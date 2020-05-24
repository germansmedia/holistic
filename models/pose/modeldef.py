from tensorflow.keras import datasets,layers,models,losses,optimizers,Model
import csv
import cv2
import numpy as np
from scipy.optimize import curve_fit
import math

class Data:
    def __init__(self,image_name,pred,screen,pos,head,ndc,light,background_color,light_color,ambient_color,skin_color):
        self.image_name = image_name
        self.pred = pred
        self.screen = screen
        self.pos = pos
        self.head = head
        self.ndc = ndc
        self.light = light
        self.background_color = background_color
        self.light_color = light_color
        self.ambient_color = ambient_color
        self.skin_color = skin_color

def load_data(dir_name,params):
    inputs = []
    outputs = []
    datas = []
    size = params['size']
    with open(dir_name + 'files.csv',newline='') as file:
        for row in csv.reader(file):
            r = 0
            image_name = row[r]
            r += 1
            pred = (float(row[r]),float(row[r + 1]))
            r += 2
            screen = (float(row[r]),float(row[r + 1]))
            r += 2
            pos = (float(row[r]),float(row[r + 1]),float(row[r + 2]))
            r += 3
            head = (float(row[r]),float(row[r + 1]))
            r += 2
            ndc = (float(row[r]),float(row[r + 1]),float(row[r + 2]))
            r += 3
            light = (float(row[r]),float(row[r + 1]))
            r += 2
            background_color = (float(row[r]),float(row[r + 1]),float(row[r + 2]))
            r += 3
            light_color = (float(row[r]),float(row[r + 1]),float(row[r + 2]))
            r += 3
            ambient_color = (float(row[r]),float(row[r + 1]),float(row[r + 2]))
            r += 3
            skin_color = (float(row[r]),float(row[r + 1]),float(row[r + 2]))
            r += 3
            image = cv2.imread(dir_name + image_name)
            image = image.astype(np.float32)
            image = np.multiply(image,1.0 / 255.0)
            inputs.append(image)
            dx = screen[0] - pred[0]
            dy = screen[1] - pred[1]
            output = [dx,dy,ndc[2],head[0],head[1],skin_color[0],skin_color[1],skin_color[2]]
            outputs.append(output)
            datas.append(Data(image_name,pred,screen,pos,head,ndc,light,background_color,light_color,ambient_color,skin_color))
    inputs = np.array(inputs)
    outputs = np.array(outputs)
    return inputs,outputs,datas

def slash_data(inputs,outputs,q):
    if isinstance(q,float):
        q = int(q * inputs.shape[0])
    return inputs[q:],outputs[q:],inputs[:q],outputs[:q]

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


def make_model(params):
    size = params['size']
    filters = params['filters']
    modules = params['modules']
    
    inputs = layers.Input(shape=(size,size,3))

    a = inception(inputs,filters)
    a = reduction(a,filters)
    a = inception(a,filters)
    a = reduction(a,filters)
    a = inception(a,filters)
    a = reduction(a,filters)
    a = inception(a,filters)
    #a = reduction(a,filters)
    #a = inception(a,filters)
    #a = reduction(a,filters)
    #a = inception(a,filters)
    #a = reduction(a,filters)
    #a = inception(a,filters)
    a = layers.Flatten()(a)

    outputs = layers.Dense(8)(a)

    model = Model(inputs=inputs,outputs=outputs)
    model.compile(
        optimizer=optimizers.Nadam(learning_rate=params['rate']),
        loss='mean_squared_error',
        metrics=['acc'])
    model.summary()
    return model

def load_model(params,name):
    model = make_model(params)
    model.load_weights(name)

def run_experiment(inputs_train,outputs_train,inputs_val,outputs_val,params):
    model = make_model(params)
    history = model.fit(
        x=inputs_train,
        y=outputs_train,
        epochs=params['epochs'],
        batch_size=params['batch_size'],
        validation_data=[inputs_val,outputs_val],
        verbose=2,
    )
    return history,model
