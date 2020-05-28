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

class PoseModel:

    def __init__(self,params,weights_name=None):

        self.params = params

        # CNN encoding helper
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

        # CNN reduction helper
        def reduction(inputs,f):
            a = layers.Conv2D(f,(3,3),strides=(2,2),activation='relu')(inputs)
            return a

        print('initializing model...')

        # input layer
        inputs = layers.Input(shape=(self.params.cutout_size,self.params.cutout_size,3))

        # encoding/reduction stack
        a = inception(inputs,self.params.pose_filters)
        a = reduction(a,self.params.pose_filters)
        a = inception(a,self.params.pose_filters)
        a = reduction(a,self.params.pose_filters)
        a = inception(a,self.params.pose_filters)
        a = reduction(a,self.params.pose_filters)
        a = inception(a,self.params.pose_filters)
        a = reduction(a,self.params.pose_filters)
        a = inception(a,self.params.pose_filters)
        a = layers.Flatten()(a)

        # output layer
        outputs = layers.Dense(5)(a)

        # setup model
        self.model = Model(inputs=inputs,outputs=outputs)
        self.model.compile(
            optimizer=optimizers.Nadam(learning_rate=self.params.pose_rate),
            loss='mean_squared_error',
            metrics=['acc'])

        self.model.summary()

        # load weights, if needed
        if weights_name != None:
            print('loading model weights...')
            self.model.load_weights(weights_name)

    def infer(self,inputs):
        return self.model.predict(tf.expand_dims(inputs,0),steps=1)[0]

### everything below here is only for training and testing the network

class PoseDataset:

    class Instance:
        def __init__(self,image_name,center,screen,ndc,head_pos,head_dir,light_dir,light_color,ambient_color,skin_color):
            self.image_name = image_name
            self.center = center
            self.screen = screen
            self.ndc = ndc
            self.head_pos = head_pos
            self.head_dir = head_dir
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

                # load image as input
                image = np.multiply(cv2.imread(path + name).astype(np.float32),1.0 / 255.0)
                inputs.append(image)

                # prepare output
                dx = screen[0] - center[0]
                dy = screen[1] - center[1]
                output = [dx,dy,ndc[2],head_dir[0],head_dir[1]]
                outputs.append(output)

                # prepare instance
                instances.append(PoseDataset.Instance(name,center,screen,ndc,head_pos,head_dir,light_dir,light_color,ambient_color,skin_color))

        # convert inputs and outputs to numpy arrays
        self.inputs = np.array(inputs)
        self.outputs = np.array(outputs)

        self.instances = instances

class PoseTraining:

    def __init__(self,params,path,csv_name):

        self.params = params

        self.dataset = PoseDataset(params,path,csv_name)

        print('splitting dataset into training and validation parts...')
        n = int(0.2 * self.dataset.inputs.shape[0])
        self.train_inputs = self.dataset.inputs[n:]
        self.train_outputs = self.dataset.outputs[n:]
        self.train_instances = self.dataset.instances[n:]
        self.val_inputs = self.dataset.inputs[:n]
        self.val_outputs = self.dataset.outputs[:n]
        self.val_instances = self.dataset.instances[:n]

    def train(self,weights_name):

        model = PoseModel(self.params)

        print('fitting model...')
        model.model.fit(
            x=self.train_inputs,
            y=self.train_outputs,
            epochs=self.params.pose_epochs,
            batch_size=self.params.pose_batch_size,
            validation_data=[self.val_inputs,self.val_outputs],
            verbose=2,
        )

        print('saving weights...')
        model.model.save_weights(weights_name)

    def train_more(self,weights_name):

        model = PoseModel(self.params,weights_name)

        print('fitting model...')
        model.model.fit(
            x=self.train_inputs,
            y=self.train_outputs,
            epochs=self.params.pose_epochs,
            batch_size=self.params.pose_batch_size,
            validation_data=[self.val_inputs,self.val_outputs],
            verbose=2
        )

        print('saving weights...')
        model.model.save_weights(weights_name)

class PoseTesting:

    def __init__(self,params,weights_name):

        self.params = params

        self.model = PoseModel(self.params,weights_name)

        self.errors = []
        self.error_names = ['screen.x','screen.y','ndc.x','ndc.y','ndc.z','head_pos.x','head_pos.y','head_pos.z','head_dir.y','head_dir.p','skin.r','skin.g','skin.b']

    def run_test(self,path,csv_name):

        # projection matrix coordinates (from commedia output)
        mxx = 2.7990382
        myy = 3.7320509
        mzz = -1.0020020
        mzw = -1.0
        mwz = -0.2002002
        mww = 0.0

        # prepare dataset for testing
        dataset = PoseDataset(self.params,path,csv_name)

        # for each instance
        for i in range(0,len(dataset.inputs)):

            print('    {} / {}'.format(i,len(dataset.inputs) - 1))

            # run the network
            result = self.model.infer(dataset.inputs[i])

            dx = result[0]
            dy = result[1]
            ndcz = result[2]
            heady = result[3]
            headp = result[4]

            # reconstruct screen coordinates
            screenx = dataset.instances[i].center[0] + dx
            screeny = dataset.instances[i].center[1] + dy

            escreenx = dataset.instances[i].screen[0] - screenx
            escreeny = dataset.instances[i].screen[1] - screeny

            # convert to NDC
            ndcx = 2.0 * screenx / self.params.frame_width - 1.0
            ndcy = 1.0 - 2.0 * screeny / self.params.frame_height

            endcx = dataset.instances[i].ndc[0] - ndcx
            endcy = dataset.instances[i].ndc[1] - ndcy
            endcz = dataset.instances[i].ndc[2] - ndcz

            # reconstruct 3D head position
            z = (mzw - mww * ndcz) / (mwz * ndcz - mzz)
            homw = mwz * z + mww
            x = homw * ndcx / mxx
            y = homw * ndcy / myy
            ex = dataset.instances[i].head_pos[0] - x
            ey = dataset.instances[i].head_pos[1] - y
            ez = dataset.instances[i].head_pos[2] - z

            # head direction
            eheady = dataset.instances[i].head_dir[0] - heady
            eheadp = dataset.instances[i].head_dir[1] - headp

            self.errors.append((escreenx,escreeny,endcx,endcy,endcz,ex,ey,ez,eheady,eheadp))

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
            print('    {:>16}: {:>6.3f} +/- {:>6.3f}'.format(self.error_names[i],avg[i],stddev[i]))

if __name__ == '__main__':
    
    # get parameters for this stage
    params = Params('./params.yaml')

    # define names of the paths and files
    weights_name = './pose.h5'

    path1data = './data1/'
    csv1data = './data1/files.csv'
    path1test = './test1/'
    csv1test = './test1/files.csv'
    
    # you need a command this time
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

    # train and train more
    if sys.argv[1] == 'train':
        
        # initialize training environment around ./data0/
        training = PoseTraining(params,path1data,csv1data)

        # either continue training or train from scratch
        if (len(sys.argv) > 2) and (sys.argv[2] == 'more'):
            training.train_more(weights_name)
        else:
            training.train(weights_name)

    # test
    elif sys.argv[1] == 'test':

        # initialize testing environment
        testing = PoseTesting(params,weights_name)

        # run test cycle on ./test0/, and show results
        print('running test cycle for statistics...')
        testing.run_test(path1test,csv1test)
