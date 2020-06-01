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

def inception(inputs,filters):
    a = layers.Conv2D(filters,(1,1),activation='relu',padding='same')(inputs)
    b = layers.Conv2D(filters,(1,1),activation='relu',padding='same')(inputs)
    b = layers.Conv2D(filters,(3,3),activation='relu',padding='same')(b)
    c = layers.Conv2D(filters,(1,1),activation='relu',padding='same')(inputs)
    c = layers.Conv2D(filters,(3,3),activation='relu',padding='same')(c)
    c = layers.Conv2D(filters,(3,3),activation='relu',padding='same')(c)
    #d = layers.Conv2D(filters,(1,1),activation='relu',padding='same')(inputs)
    #d = layers.Conv2D(filters,(3,3),activation='relu',padding='same')(d)
    #d = layers.Conv2D(filters,(3,3),activation='relu',padding='same')(d)
    #d = layers.Conv2D(filters,(3,3),activation='relu',padding='same')(d)
    e = layers.MaxPooling2D((3,3),strides=(1,1),padding='same')(inputs)
    e = layers.Conv2D(filters,(1,1),activation='relu',padding='same')(e)
    #return layers.Concatenate(axis=3)([a,b,c,d,e])
    return layers.Concatenate(axis=3)([a,b,c,e])

def reduction(inputs,f):
    a = layers.Conv2D(f,(3,3),strides=(2,2),activation='relu')(inputs)
    return a

def create(cutout_size,filters,rate):
    inputs = layers.Input(shape=(cutout_size,cutout_size,4))
    a = inception(inputs,filters)
    a = reduction(a,filters)
    a = inception(a,filters)
    a = reduction(a,filters)
    a = inception(a,filters)
    a = reduction(a,filters)
    a = inception(a,filters)
    a = layers.Flatten()(a)
    outputs = layers.Dense(5)(a)
    model = Model(inputs=inputs,outputs=outputs)
    model.compile(
        optimizer=optimizers.Nadam(learning_rate=rate),
        loss='mean_absolute_error',
        metrics=['acc'])
    model.summary()
    return model

def infer(model,inputs):
    return model.predict(tf.expand_dims(inputs,0),steps=1)[0]

def load_dataset(path,csv_name):
    inputs = []
    outputs = []
    instances = []
    with open(csv_name,newline='') as file:
        for row in csv.reader(file):
            r = 0
            name = row[r]
            r += 1
            center = (float(row[r]),float(row[r + 1]))
            r += 2
            screen = (float(row[r]),float(row[r + 1]))
            r += 2
            ndc = (float(row[r]),float(row[r + 1]),float(row[r + 2]))
            r += 3
            head_pos = (float(row[r]),float(row[r + 1]),float(row[r + 2]))
            r += 3
            head_dir = (float(row[r]),float(row[r + 1]))
            r += 2
            light_dir = (float(row[r]),float(row[r + 1]))
            r += 2
            light_color = (float(row[r]),float(row[r + 1]),float(row[r + 2]))
            r += 3
            ambient_color = (float(row[r]),float(row[r + 1]),float(row[r + 2]))
            r += 3
            skin_color = (float(row[r]),float(row[r + 1]),float(row[r + 2]))
            r += 3
            image = np.multiply(cv2.imread(path + name,cv2.IMREAD_UNCHANGED).astype(np.float32),1.0 / 255.0)
            inputs.append(image)
            dx = screen[0] - center[0]
            dy = screen[1] - center[1]
            output = [dx,dy,ndc[2],head_dir[0],head_dir[1]]
            outputs.append(output)
            instances.append((name,center,screen,ndc,head_pos,head_dir,light_dir,light_color,ambient_color,skin_color))
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

def test(model,dataset,frame_width,frame_height,cutout_size):
    errors = []
    error_names = ['screen.x','screen.y','ndc.x','ndc.y','ndc.z','head_pos.x','head_pos.y','head_pos.z','head_dir.y','head_dir.p','skin.r','skin.g','skin.b']

    mxx = 2.7990382
    myy = 3.7320509
    mzz = -1.0020020
    mzw = -1.0
    mwz = -0.2002002
    mww = 0.0

    for i in range(0,len(dataset[0])):
        print('    {} / {}'.format(i,len(dataset[0]) - 1))
        inference = infer(model,dataset[0][i])
        dx = inference[0]
        dy = inference[1]
        ndcz = inference[2]
        heady = inference[3]
        headp = inference[4]
        screenx = dataset[2][i][1][0] + dx
        screeny = dataset[2][i][1][1] + dy
        escreenx = dataset[2][i][2][0] - screenx
        escreeny = dataset[2][i][2][1] - screeny
        ndcx = 2.0 * screenx / frame_width - 1.0
        ndcy = 1.0 - 2.0 * screeny / frame_height
        endcx = dataset[2][i][3][0] - ndcx
        endcy = dataset[2][i][3][1] - ndcy
        endcz = dataset[2][i][3][2] - ndcz
        z = (mzw - mww * ndcz) / (mwz * ndcz - mzz)
        homw = mwz * z + mww
        x = homw * ndcx / mxx
        y = homw * ndcy / myy
        ex = dataset[2][i][4][0] - x
        ey = dataset[2][i][4][1] - y
        ez = dataset[2][i][4][2] - z
        eheady = dataset[2][i][5][0] - heady
        eheadp = dataset[2][i][5][1] - headp
        errors.append((escreenx,escreeny,endcx,endcy,endcz,ex,ey,ez,eheady,eheadp))
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

if __name__ == '__main__':
    params = Params('./params.yaml')

    weights_name = './pose.h5'

    path1data = './data1/'
    csv1data = './data1/files.csv'
    path1test = './test1/'
    csv1test = './test1/files.csv'
    
    if len(sys.argv) < 2:
        print('usage:')
        print('')
        print('    python3 pose.py <command>')
        print('')
        print('where command is:')
        print('    train       - start training')
        print('    train more  - improve training')
        print('    test        - numerically test the network')
        exit(-1)

    if sys.argv[1] == 'train':
        print('creating model...')
        model = create(params.cutout_size,params.pose_filters,params.pose_rate)
        print('loading data1 dataset...')
        dataset = load_dataset(path1data,csv1data)
        if (len(sys.argv) > 2) and (sys.argv[2] == 'more'):
            print('loading old weights...')
            model.load_weights(weights_name)
        print('training...')
        train(model,dataset,params.pose_epochs,params.pose_batch_size)
        print('saving new weights...')
        model.save_weights(weights_name)

    elif sys.argv[1] == 'test':
        print('creating model...')
        model = create(params.cutout_size,params.pose_filters,params.pose_rate)
        print('loading weights...')
        model.load_weights(weights_name)
        print('loading test0 dataset...')
        dataset = load_dataset(path1test,csv1test)
        print('measuring statistics...')
        test(model,dataset,params.frame_width,params.frame_height,params.cutout_size)
