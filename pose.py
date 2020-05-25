#!/usr/bin/python3
# -*- coding: utf-8 -*-

import sys
from math import *
import numpy as np
import yaml
import tensorflow as tf
from tensorflow.keras import datasets,layers,models,losses,optimizers,Model

def inception(inputs,filters):
    a = layers.Conv2D(filters,(1,1),activation='relu',padding='same')(inputs)
    b = layers.Conv2D(filters,(1,1),activation='relu',padding='same')(inputs)
    b = layers.Conv2D(filters,(3,3),activation='relu',padding='same')(b)
    c = layers.Conv2D(filters,(1,1),activation='relu',padding='same')(inputs)
    c = layers.Conv2D(filters,(3,3),activation='relu',padding='same')(c)
    c = layers.Conv2D(filters,(3,3),activation='relu',padding='same')(c)
    d = layers.Conv2D(filters,(1,1),activation='relu',padding='same')(inputs)
    d = layers.Conv2D(filters,(3,3),activation='relu',padding='same')(d)
    d = layers.Conv2D(filters,(3,3),activation='relu',padding='same')(d)
    d = layers.Conv2D(filters,(3,3),activation='relu',padding='same')(d)
    e = layers.MaxPooling2D((3,3),strides=(1,1),padding='same')(inputs)
    e = layers.Conv2D(filters,(1,1),activation='relu',padding='same')(e)
    return layers.Concatenate(axis=3)([a,b,c,d,e])

def reduction(inputs,f):
    a = layers.Conv2D(f,(3,3),strides=(2,2),activation='relu')(inputs)
    return a

class ModelPOSE():

    def __init__(self):
        self.params = yaml.load(open('pose.yaml','r'))
        size = self.params['size']
        width = self.params['width']
        height = self.params['height']
        filters = self.params['filters']
        modules = self.params['modules']
        rate = self.params['rate']
        inputs = layers.Input(shape=(size,size,3))
        a = inception(inputs,filters)
        a = reduction(a,filters)
        a = inception(a,filters)
        a = reduction(a,filters)
        a = inception(a,filters)
        a = reduction(a,filters)
        a = inception(a,filters)
        a = layers.Flatten()(a)
        outputs = layers.Dense(8)(a)
        self.model = Model(inputs=inputs,outputs=outputs)
        self.model.compile(
            optimizer=optimizers.Nadam(learning_rate=rate),
            loss='mean_squared_error',
            metrics=['acc'])
        self.model.load_weights('pose.h5')

    def __call__(self,inputs):
        return self.model.predict(tf.expand_dims(inputs,0),steps=1)[0]
