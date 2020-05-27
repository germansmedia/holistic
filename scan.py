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

class ScanParams:

    def __init__(self,name):
        params = yaml.load(open(name,'r'))
        self.width = params['width']
        self.height = params['height']
        self.factor = params['factor']
        self.filters = params['filters']
        self.modules = params['modules']
        self.rate = params['rate']
        self.batch_size = params['batch_size']
        self.epochs = params['epochs']

class ScanModel:

    def __init__(self,params,weights_name=None):

        self.params = params

        # CNN encoding helper
        def encoding(a,filters,modules):
            for i in range(0,modules):
                a = layers.Conv2D(filters,(3,3),activation='relu',padding='same')(a)
            return a

        # CNN reduction helper
        def reduction(a):
            return layers.MaxPooling2D((2,2))(a)

        print('initializing model...')

        # input layer
        inputs = layers.Input(shape=(self.params.height,self.params.width,3))

        # encoding/reduction stack
        a = encoding(inputs,self.params.filters,self.params.modules)
        cur = 1
        while cur < self.params.factor:
            a = reduction(a)
            a = encoding(a,self.params.filters,self.params.modules)
            cur *= 2
        a = encoding(a,self.params.filters,self.params.modules)

        # output layer
        outputs = layers.Conv2D(1,(1,1),activation='sigmoid',padding='same')(a)

        # setup model
        self.model = Model(inputs=inputs,outputs=outputs)
        self.model.compile(
            optimizer=optimizers.Nadam(learning_rate=self.params.rate),
            loss='binary_crossentropy',
            metrics=['acc'])

        # load weights, if needed
        if weights_name != None:
            print('loading model weights...')
            self.model.load_weights(weights_name)

    def infer(self,inputs):
        return self.model.predict(tf.expand_dims(inputs,0),steps=1)[0]

### everything below here is only for training and testing the network

class ScanDataset:

    class Instance:
        def __init__(self,image_name,head_pos,head_dir,ndc,screen,light_dir,light_color,ambient_color,skin_color):
            self.image_name = image_name
            self.head_pos = head_pos
            self.head_dir = head_dir
            self.ndc = ndc
            self.screen = screen
            self.light_dir = light_dir
            self.light_color = light_color
            self.ambient_color = ambient_color
            self.skin_color = skin_color

    def __init__(self,params,path,csv_name):

        self.params = params

        print('loading {} dataset...'.format(path))

        # prepare inputs, outputs and instances
        inputs = []
        outputs = []
        instances = []

        # read all rows in the CSV
        with open(csv_name,newline='') as file:
            for row in csv.reader(file):

                # read one row
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

                # load image as input
                image = np.multiply(cv2.imread(path + name).astype(np.float32),1.0 / 255.0)

                # prepare empty output
                rwidth = int(math.floor(self.params.width / self.params.factor))
                rheight = int(math.floor(self.params.height / self.params.factor))
                output = np.zeros((rheight,rwidth),dtype=np.float32)

                # take the screen coordinates as output
                x = int(math.floor(screen[0] / self.params.factor))
                y = int(math.floor(screen[1] / self.params.factor))

                # if they really fit inside the output
                if (x >= 0) and (x < rwidth) and (y >= 0) and (y < rheight):

                    # light up that point
                    output[y][x] = 1.0

                    # and append input, output and instance
                    inputs.append(image)
                    outputs.append(output)
                    instances.append(ScanDataset.Instance(name,head_pos,head_dir,ndc,screen,light_dir,light_color,ambient_color,skin_color))

        # convert inputs and outputs to numpy arrays
        self.inputs = np.array(inputs)
        self.outputs = np.array(outputs)

        self.instances = instances

class ScanTraining:

    def __init__(self,params,path,csv_name):

        self.params = params

        self.dataset = ScanDataset(params,path,csv_name)

        print('splitting dataset into training and validation parts...')
        n = int(0.2 * self.dataset.inputs.shape[0])
        self.train_inputs = self.dataset.inputs[n:]
        self.train_outputs = self.dataset.outputs[n:]
        self.train_instances = self.dataset.instances[n:]
        self.val_inputs = self.dataset.inputs[:n]
        self.val_outputs = self.dataset.outputs[:n]
        self.val_instances = self.dataset.instances[:n]

    def train(self,weights_name):

        model = ScanModel(self.params)

        print('fitting model...')
        model.model.fit(
            x=self.train_inputs,
            y=self.train_outputs,
            epochs=self.params.epochs,
            batch_size=self.params.batch_size,
            validation_data=[self.val_inputs,self.val_outputs],
            verbose=2,
        )

        print('saving weights...')
        model.model.save_weights(weights_name)

    def train_more(self,weights_name):

        model = ScanModel(self.params,weights_name)

        print('fitting model...')
        model.model.fit(
            x=self.train_inputs,
            y=self.train_outputs,
            epochs=self.params.epochs,
            batch_size=self.params.batch_size,
            validation_data=[self.val_inputs,self.val_outputs],
            verbose=2
        )

        print('saving weights...')
        model.model.save_weights(weights_name)

class ScanTesting:

    def __init__(self,params,weights_name):

        self.params = params

        self.model = ScanModel(self.params,weights_name)

        self.errors = []

        self.generate_writer = None
        self.generate_n = 0
        self.generate_path = None

    def run_test(self,path,csv_name,instance_proc):

        # prepare dataset for testing
        dataset = ScanDataset(self.params,path,csv_name)

        # define threshold
        threshold = 0.3

        # for each instance
        rwidth = int(math.floor(self.params.width / self.params.factor))
        rheight = int(math.floor(self.params.height / self.params.factor))
        for i in range(0,len(dataset.inputs)):

            # run the network
            result = self.model.infer(dataset.inputs[i])

            # find the cell with the highest result
            px = -1
            py = -1
            highest = 0.0
            for y in range(0,rheight):
                for x in range(0,rwidth):
                    if result[y][x][0] > highest:
                        highest = result[y][x][0]
                        px = x
                        py = y

            if highest > threshold:

                print('    {} / {}'.format(i,len(dataset.inputs) - 1))

                # adjust subcell accuracy
                if py > 0:
                    u = result[py - 1][px][0]
                else:
                    u = 0.0
                if py < rheight - 1:
                    d = result[py + 1][px][0]
                else:
                    d = 0.0
                if px > 0:
                    l = result[py][px - 1][0]
                else:
                    l = 0.0
                if px < rwidth - 1:
                    r = result[py][px + 1][0]
                else:
                    r = 0.0
                totalx = highest + l + r
                totaly = highest + u + d
                ax = (r - l) / totalx
                ay = (d - u) / totaly

                px = int((px + ax) * self.params.factor)
                py = int((py + ay) * self.params.factor)

                # and run the per-instance test
                instance_proc(dataset.inputs[i],dataset.instances[i],(px,py))

            else:
                print('    {} / {} not found'.format(i,len(dataset.inputs) - 1))

    def collect_errors(self,image,instance,inferred):

        # calculate screen-space error
        ex = instance.screen[0] - inferred[0]
        ey = instance.screen[1] - inferred[1]

        # append to error list
        self.errors.append((ex,ey))

    def generate_cutout(self,image,instance,inferred):

        # create cutout
        cutout = np.zeros((129,129,3),np.float32)
        for y in range(-64,65):
            cy = int(inferred[1] + y)
            if (cy >= 0) and (cy < self.params.height):
                for x in range(-64,65):
                    cx = int(inferred[0] + x)
                    if (cx >= 0) and (cx < self.params.width):
                        cutout[y + 64,x + 64] = image[cy,cx]
        
        # save cutout
        cutout = np.multiply(cutout,255.0)
        cutout = cutout.astype(np.uint8)
        name = '{:05}.bmp'.format(self.generate_n)
        cv2.imwrite(self.generate_path + name,cutout)

        # and write a corresponding line in the csv
        self.generate_writer.writerow([
            name,
            inferred[0],inferred[1],
            instance.screen[0],instance.screen[1],
            instance.head_pos[0],instance.head_pos[1],instance.head_pos[2],
            instance.head_dir[0],instance.head_dir[1],
            instance.ndc[0],instance.ndc[1],instance.ndc[2],
            instance.light_dir[0],instance.light_dir[1],
            instance.light_color[0],instance.light_color[1],instance.light_color[2],
            instance.ambient_color[0],instance.ambient_color[1],instance.ambient_color[2],
            instance.skin_color[0],instance.skin_color[1],instance.skin_color[2]
        ])

        self.generate_n += 1

    def process_errors(self):

        # get number of samples
        total = len(self.errors)

        # get number of parameters
        n = len(self.errors[0])

        # calculate averages
        avg = []
        for i in range(0,n):
            avg.append(0.0)
        for error in self.errors:
            for i in range(0,n):
                avg[i] += error[i]
        for i in range(0,n):
            avg[i] /= total

        # calculate standard deviations
        dif = []
        for i in range(0,n):
            dif.append(0.0)
        for error in self.errors:
            for i in range(0,n):
                d = error[i] - avg[i]
                dif[i] += d * d
        almost_total = total - 1.0
        for i in range(0,n):
            dif[i] /= almost_total
        stddev = []
        for i in range(0,n):
            stddev.append(math.sqrt(dif[i]))

        # and show the results
        print('statistics:')
        print('    # of samples: {}'.format(total))
        for i in range(0,n):
            print('    parameter {:2}: {:5.3} +/- {:5.3}'.format(i,avg[i],stddev[i]))

if __name__ == '__main__':
    
    # get parameters for this stage
    params = ScanParams('./scan.yaml')

    # define names of the paths and files
    weights_name = './scan.h5'

    path0data = './data0/'
    csv0data = './data0/files.csv'
    path0test = './test0/'
    csv0test = './test0/files.csv'

    path1data = './data1/'
    csv1data = './data1/files.csv'
    path1test = './test1/'
    csv1test = './test1/files.csv'
    
    # you need a command this time
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

    # train and train more
    if sys.argv[1] == 'train':
        
        # initialize training environment around ./data0/
        training = ScanTraining(params,path0data,csv0data)

        # either continue training or train from scratch
        if (len(sys.argv) > 2) and (sys.argv[2] == 'more'):
            training.train_more(weights_name)
        else:
            training.train(weights_name)

    # test
    elif sys.argv[1] == 'test':

        # initialize testing environment
        testing = ScanTesting(params,weights_name)

        # run test cycle on ./test0/, and show results
        print('running test cycle for statistics...')
        testing.run_test(path0test,csv0test,testing.collect_errors)
        testing.process_errors()

    # generate
    elif sys.argv[1] == 'generate':

        # initialize testing environment
        testing = ScanTesting(params,weights_name)

        # run test cycle on ./data0/, and create cutouts in ./data1/
        print('running test cycle to generate cutouts for {}...',path1data)
        if os.path.exists(path1data):
            shutil.rmtree(path1data)
        os.mkdir(path1data)
        with open(csv1data,'w',newline='') as file:
            testing.generate_writer = csv.writer(file)
            testing.generate_n = 0
            testing.generate_path = path1data
            testing.run_test(path0data,csv0data,testing.generate_cutout)

        # run test cycle on ./test0/, and create cutouts in ./test1/
        print('running test cycle to generate cutouts for {}...',path1test)
        if os.path.exists(path1test):
            shutil.rmtree(path1test)
        os.mkdir(path1test)
        with open(csv1test,'w',newline='') as file:
            testing.generate_writer = csv.writer(file)
            testing.generate_n = 0
            testing.generate_path = path1test
            testing.run_test(path0test,csv0test,testing.generate_cutout)
