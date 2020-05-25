from tensorflow.keras import datasets,layers,models,losses,optimizers,Model
import csv
import cv2
import numpy as np
from scipy.optimize import curve_fit
import math

class Data:
    def __init__(self,image_name,pos,head,ndc,screen,light,background_color,light_color,ambient_color,skin_color):
        self.image_name = image_name
        self.pos = pos
        self.head = head
        self.ndc = ndc
        self.screen = screen
        self.light = light
        self.background_color = background_color
        self.light_color = light_color
        self.ambient_color = ambient_color
        self.skin_color = skin_color

def load_data(dir_name,params):
    inputs = []
    outputs = []
    datas = []
    width = params['width']
    height = params['height']
    factor = params['factor']
    with open(dir_name + 'files.csv',newline='') as file:
        for row in csv.reader(file):
            r = 0
            image_name = row[r]
            r += 1
            pos = (float(row[r]),float(row[r + 1]),float(row[r + 2]))
            r += 3
            head = (float(row[r]),float(row[r + 1]))
            r += 2
            ndc = (float(row[r]),float(row[r + 1]),float(row[r + 2]))
            r += 3
            screen = (float(row[r]),float(row[r + 1]))
            r += 2
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
            output = np.zeros((int(height / factor),int(width / factor)),dtype=np.float32)
            x = int(round(screen[0] / factor))
            y = int(round(screen[1] / factor))
            if (x >= 0) and (x < (width / factor)) and (y >= 0) and (y < (height / factor)):
                output[y][x] = 1.0
                inputs.append(image)
                outputs.append(output)
                datas.append(Data(image_name,pos,head,ndc,screen,light,background_color,light_color,ambient_color,skin_color))
    inputs = np.array(inputs)
    outputs = np.array(outputs)
    return inputs,outputs,datas

def slash_data(inputs,outputs,q):
    if isinstance(q,float):
        q = int(q * inputs.shape[0])
    return inputs[q:],outputs[q:],inputs[:q],outputs[:q]

def encoding(a,filters,modules):
    for i in range(0,modules):
        a = layers.Conv2D(filters,(3,3),activation='relu',padding='same')(a)
    return a

def reduction(a):
    return layers.MaxPooling2D((2,2))(a)

def make_model(params):
    width = params['width']
    height = params['height']
    factor = params['factor']
    filters = params['filters']
    modules = params['modules']
    
    inputs = layers.Input(shape=(height,width,3))
    print('shape: {}'.format(inputs.get_shape()))

    a = encoding(inputs,filters,modules)
    cur = 1
    while cur < factor:
        a = reduction(a)
        a = encoding(a,filters,modules)
        cur *= 2
    a = encoding(a,filters,modules)
    outputs = layers.Conv2D(1,(1,1),activation='sigmoid',padding='same')(a)

    model = Model(inputs=inputs,outputs=outputs)
    model.compile(
        optimizer=optimizers.Nadam(learning_rate=params['rate']),
        loss='binary_crossentropy',
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
