#!/usr/bin/python3
# -*- coding: utf-8 -*-

import sys
from math import *
import numpy as np
import yaml
import tensorflow as tf
from tensorflow.keras import datasets,layers,models,losses,optimizers,Model

class ModelFSCAN():

    def __init__(self):
        self.params = yaml.load(open('fscan.yaml','r'))
        width = self.params['width']
        height = self.params['height']
        factor = self.params['factor']
        filters = self.params['filters']
        modules = self.params['modules']
        rate = self.params['rate']
        inputs = layers.Input(shape=(height,width,3))
        a = inputs
        for i in range(0,modules):
            a = layers.Conv2D(filters,(3,3),activation='relu',padding='same')(a)
        cur = 1
        while cur < factor:
            a = layers.MaxPooling2D((2,2))(a)
            for i in range(0,modules):
                a = layers.Conv2D(filters * cur,(3,3),activation='relu',padding='same')(a)
            cur *= 2
        for i in range(0,modules):
            a = layers.Conv2D(filters * factor,(3,3),activation='relu',padding='same')(a)
        outputs = layers.Conv2D(1,(1,1),activation='sigmoid',padding='same')(a)
        self.model = Model(inputs=inputs,outputs=outputs)
        self.model.compile(
            optimizer=optimizers.Nadam(learning_rate=rate),
            loss='binary_crossentropy',
            metrics=['acc'])
        self.model.load_weights('fscan.h5')

    def __call__(self,inputs):
        return self.model.predict(tf.expand_dims(inputs,0),steps=1)[0]
