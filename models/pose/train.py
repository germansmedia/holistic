import tensorflow as tf
from tensorflow.keras import datasets,layers,models,losses,Model
import csv
import cv2
import sys
import yaml
import numpy as np
import modeldef

if __name__ == "__main__":
    params = yaml.load(open('pose.yaml','r'))
    inputs,outputs,datas = modeldef.load_data('../../data1/',params)
    inputs_train,outputs_train,inputs_val,outputs_val = modeldef.slash_data(inputs,outputs,0.2)
    model = modeldef.make_model(params)
    if len(sys.argv) > 1:
        print("loading existing weights")
        model.load_weights('pose.h5')
    history = model.fit(
        x=inputs_train,
        y=outputs_train,
        epochs=params['epochs'],
        batch_size=params['batch_size'],
        validation_data=[inputs_val,outputs_val],
        verbose=2,
    )
    model.save_weights('pose.h5')
