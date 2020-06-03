#!/usr/bin/python3
# -*- coding: utf-8 -*-

import sys
import os
import math
import yaml
import csv
import cv2
import shutil
import numpy as np
import tensorflow as tf
from tensorflow.keras import datasets,layers,models,losses,optimizers,Model
from params import *

def encoding(a,filters,modules):
    for i in range(0,modules):
        a = layers.Conv2D(filters,(3,3),activation='relu',padding='same')(a)
    return a

def reduction(a):
    return layers.MaxPooling2D((2,2))(a)

def create(frame_width,frame_height,depth,factor,filters,modules,rate):
    if depth:
        inputs = layers.Input(shape=(frame_height,frame_width,4))
    else:
        inputs = layers.Input(shape=(frame_height,frame_width,3))
    a = encoding(inputs,filters,modules)
    cur = 1
    while cur < factor:
        a = reduction(a)
        a = encoding(a,filters,modules)
        cur *= 2
    outputs = layers.Conv2D(1,(1,1),activation='sigmoid',padding='same')(a)
    model = Model(inputs=inputs,outputs=outputs)
    model.compile(
        optimizer=optimizers.Nadam(learning_rate=rate),
        loss='binary_crossentropy',
        metrics=['accuracy'],
    )
    model.summary()
    return model

def infer(model,inputs):
    return model.predict(tf.expand_dims(inputs,0),steps=1)[0]

def load_dataset(frame_width,frame_height,depth,factor,path,csv_name):
    rwidth = int(math.floor(frame_width / factor))
    rheight = int(math.floor(frame_height / factor))
    inputs = []
    outputs = []
    instances = []
    with open(csv_name,newline='') as file:
        for row in csv.reader(file):
            r = 0
            name = row[r]
            r += 1
            head_pos = (float(row[r]),float(row[r + 1]),float(row[r + 2]))
            r += 3
            head_dir = (float(row[r]),float(row[r + 1]))
            r += 2
            ndc = (float(row[r]),float(row[r + 1]),float(row[r + 2]))
            r += 3
            screen = (float(row[r]),float(row[r + 1]))
            r += 2
            light_dir = (float(row[r]),float(row[r + 1]))
            r += 2
            light_color = (float(row[r]),float(row[r + 1]),float(row[r + 2]))
            r += 3
            ambient_color = (float(row[r]),float(row[r + 1]),float(row[r + 2]))
            r += 3
            skin_color = (float(row[r]),float(row[r + 1]),float(row[r + 2]))
            r += 3
            if depth:
                image = np.multiply(cv2.imread(path + name,cv2.IMREAD_UNCHANGED).astype(np.float32),1.0 / 255.0)
            else:
                image = np.multiply(cv2.imread(path + name).astype(np.float32),1.0 / 255.0)
            output = np.zeros((rheight,rwidth,1),dtype=np.float32)
            px = int(math.floor(screen[0] / factor))
            py = int(math.floor(screen[1] / factor))
            if (px >= 0) and (px < rwidth) and (py >= 0) and (py < rheight):
                output[py,px] = 1.0
                inputs.append(image)
                outputs.append(output)
                instances.append((name,head_pos,head_dir,ndc,screen,light_dir,light_color,ambient_color,skin_color))
    return (np.array(inputs),np.array(outputs),instances)

def train(model,dataset,epochs,batch_size):
    n = int(0.2 * dataset[0].shape[0])
    x_train = dataset[0][n:]
    y_train = dataset[1][n:]
    x_val = dataset[0][:n]
    y_val = dataset[1][:n]
    history = model.fit(
        x=x_train,
        y=y_train,
        validation_data=[x_val,y_val],
        epochs=epochs,
        batch_size=batch_size,
        verbose=2,
    )

def find(image,inference,frame_width,frame_height,factor,threshold):
    rwidth = int(math.floor(frame_width / factor))
    rheight = int(math.floor(frame_height / factor))
    px = -1
    py = -1
    highest = 0.0
    for y in range(0,rheight):
        for x in range(0,rwidth):
            if inference[y,x,0] > highest:
                highest = inference[y,x,0]
                px = x
                py = y
    if highest > threshold:
        if py > 0:
            u = inference[py - 1,px,0]
        else:
            u = 0.0
        if py < rheight - 1:
            d = inference[py + 1,px,0]
        else:
            d = 0.0
        if px > 0:
            l = inference[py,px - 1,0]
        else:
            l = 0.0
        if px < rwidth - 1:
            r = inference[py,px + 1,0]
        else:
            r = 0.0
        px += (r - l) / (highest + r + l)
        py += (d - u) / (highest + d + u)
        px = int((px + 0.5) * factor)
        py = int((py + 0.5) * factor)
        return (px,py)
    else:
        return None

def test(model,dataset,frame_width,frame_height,factor,threshold):
    errors = []
    error_names = ['screen.x','screen.y']
    for i in range(0,len(dataset[0])):
        inference = infer(model,dataset[0][i])
        result = find(dataset[0][i],inference,frame_width,frame_height,factor,threshold)
        if result != None:
            print('    {} / {}'.format(i,len(dataset[0]) - 1))
            ex = dataset[2][i][4][0] - result[0]
            ey = dataset[2][i][4][1] - result[1]
            errors.append((ex,ey))
        else:
            print('    {} / {} not found'.format(i,len(dataset[0]) - 1))
    total = len(errors)
    n = len(errors[0])
    avg = []
    for i in range(0,n):
        avg.append(0.0)
    for error in errors:
        for i in range(0,n):
            avg[i] += error[i]
    for i in range(0,n):
        avg[i] /= total
    dif = []
    for i in range(0,n):
        dif.append(0.0)
    for error in errors:
        for i in range(0,n):
            d = error[i] - avg[i]
            dif[i] += d * d
    almost_total = total - 1.0
    for i in range(0,n):
        dif[i] /= almost_total
    stddev = []
    for i in range(0,n):
        stddev.append(math.sqrt(dif[i]))
    print('statistics:')
    for i in range(0,n):
        print('    {:>16}: {:>6.3f} +/- {:>6.3f}'.format(error_names[i],avg[i],stddev[i]))

def rgb2float(image):
    return np.multiply(image.astype(np.float32),1.0 / 255.0)

def rgba2float(image,image_depth):
    r,g,b = image.split()
    rgba = cv2.merge(r,g,b,image_depth)
    return np.multiply(rgba.astype(np.float32),1.0 / 255.0)

def float2rgb(image):
    return np.multiply(image,255.0).astype(np.uint8)

def generate_cutout(px,py,image,frame_width,frame_height,depth,cutout_size):
    if depth:
        cutout = np.zeros((cutout_size,cutout_size,4),np.float32)
    else:
        cutout = np.zeros((cutout_size,cutout_size,3),np.float32)
    half = int((cutout_size - 1) / 2)
    for y in range(-half,half + 1):
        cy = int(py + y)
        if (cy >= 0) and (cy < frame_height):
            for x in range(-half,half + 1):
                cx = int(px + x)
                if (cx >= 0) and (cx < frame_width):
                    cutout[y + half,x + half] = image[cy,cx]
    return cutout

def generate_cutouts(model,dataset,frame_width,frame_height,depth,factor,threshold,cutout_size,path,csv_name):
    with open(csv_name,'w',newline='') as file:
        writer = csv.writer(file)
        for i in range(0,len(dataset[0])):
            inference = infer(model,dataset[0][i])
            result = find(dataset[0][i],inference,frame_width,frame_height,factor,threshold)
            if result != None:
                print('    {} / {}'.format(i,len(dataset[0]) - 1))
                cutout = generate_cutout(result[0],result[1],dataset[0][i],frame_width,frame_height,depth,cutout_size)
                name = '{:05}.png'.format(i)
                cv2.imwrite(path + name,float2rgb(cutout))
                writer.writerow([
                    name,
                    result[0],result[1],
                    dataset[2][i][4][0],dataset[2][i][4][1],
                    dataset[2][i][3][0],dataset[2][i][3][1],dataset[2][i][3][2],
                    dataset[2][i][1][0],dataset[2][i][1][1],dataset[2][i][1][2],
                    dataset[2][i][2][0],dataset[2][i][2][1],
                    dataset[2][i][5][0],dataset[2][i][5][1],
                    dataset[2][i][6][0],dataset[2][i][6][1],dataset[2][i][6][2],
                    dataset[2][i][7][0],dataset[2][i][7][1],dataset[2][i][7][2],
                    dataset[2][i][8][0],dataset[2][i][8][1],dataset[2][i][8][2],
                ])
            else:
                print('    {} / {} not found'.format(i,len(dataset[0]) - 1))

if __name__ == '__main__':
    params = Params('./params.yaml')

    weights_name = './scan.h5'

    path0data = './data0/'
    csv0data = './data0/files.csv'
    path0test = './test0/'
    csv0test = './test0/files.csv'

    path1data = './data1/'
    csv1data = './data1/files.csv'
    path1test = './test1/'
    csv1test = './test1/files.csv'
    
    if len(sys.argv) < 2:
        print('usage:')
        print('')
        print('    python3 scan.py <command>')
        print('')
        print('where command is:')
        print('    train       - start training')
        print('    train more  - improve training')
        print('    test        - numerically test the network')
        print('    generate    - generate new dataset from cutouts')
        exit(-1)

    if sys.argv[1] == 'train':
        print('creating model...')
        model = create(params.frame_width,params.frame_height,params.depth,params.factor,params.scan_filters,params.scan_modules,params.scan_rate)
        print('loading data0 dataset...')
        dataset = load_dataset(params.frame_width,params.frame_height,params.depth,params.factor,path0data,csv0data)
        if (len(sys.argv) > 2) and (sys.argv[2] == 'more'):
            print('loading old weights...')
            model.load_weights(weights_name)
        print('training...')
        train(model,dataset,params.scan_epochs,params.scan_batch_size)
        print('saving new weights...')
        model.save_weights(weights_name)

    elif sys.argv[1] == 'test':
        print('creating model...')
        model = create(params.frame_width,params.frame_height,params.depth,params.factor,params.scan_filters,params.scan_modules,params.scan_rate)
        print('loading weights...')
        model.load_weights(weights_name)
        print('loading test0 dataset...')
        dataset = load_dataset(params.frame_width,params.frame_height,params.depth,params.factor,path0test,csv0test)
        print('measuring statistics...')
        test(model,dataset,params.frame_width,params.frame_height,params.factor,params.threshold)

    elif sys.argv[1] == 'generate':
        print('creating model...')
        model = create(params.frame_width,params.frame_height,params.depth,params.factor,params.scan_filters,params.scan_modules,params.scan_rate)
        print('loading weights...')
        model.load_weights(weights_name)
        print('loading data0 dataset...')
        dataset = load_dataset(params.frame_width,params.frame_height,params.depth,params.factor,path0data,csv0data)
        if os.path.exists(path1data):
            shutil.rmtree(path1data)
        os.mkdir(path1data)
        print('generating cutouts...')
        generate_cutouts(model,dataset,params.frame_width,params.frame_height,params.depth,params.factor,params.threshold,params.cutout_size,path1data,csv1data)
        print('loading test0 dataset...')
        dataset = load_dataset(params.frame_width,params.frame_height,params.depth,params.factor,path0test,csv0test)
        if os.path.exists(path1test):
            shutil.rmtree(path1test)
        os.mkdir(path1test)
        print('generating cutouts...')
        generate_cutouts(model,dataset,params.frame_width,params.frame_height,params.depth,params.factor,params.threshold,params.cutout_size,path1test,csv1test)

    print('done.')
