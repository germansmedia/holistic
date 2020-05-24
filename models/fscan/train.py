import tensorflow as tf
from tensorflow.keras import datasets,layers,models,losses,Model
import csv
import cv2
import sys
import numpy as np
import yaml
import modeldef

if __name__ == "__main__":
    params = yaml.load(open('fscan.yaml','r'))
    inputs,outputs,datas = modeldef.load_data('../../data0/',params)
    inputs_train,outputs_train,inputs_val,outputs_val = modeldef.slash_data(inputs,outputs,0.2)
    model = modeldef.make_model(params)
    if len(sys.argv) > 1:
        print("loading existing weights")
        model.load_weights('fscan.h5')
    history = model.fit(
        x=inputs_train,
        y=outputs_train,
        epochs=4,
        batch_size=params['batch_size'],
        validation_data=[inputs_val,outputs_val],
        verbose=2,
    )
    model.save_weights('fscan.h5')
