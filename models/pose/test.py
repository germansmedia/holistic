import tensorflow as tf
from tensorflow.keras import datasets,layers,models,Model
import csv
import cv2
import yaml
import numpy as np
import math
import modeldef

def calculate_measurement_errors(errors):
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
    return avg,stddev

if __name__ == "__main__":
    params = yaml.load(open('pose.yaml','r'))
    inputs,outputs,datas = modeldef.load_data('../../test1/',params)
    model = modeldef.make_model(params)
    model.load_weights('pose.h5')

    errors = []
    width = params['width']
    height = params['height']

    # projection and camera matrix used in image generation
    mxx = 2.7990382
    myy = 3.732051
    mzz = -1.002002
    mzw = -1.0
    mwz = -0.2002002
    mww = 0.0

    for i in range(0,len(inputs)):

        # predict the model
        result = model.predict(tf.expand_dims(inputs[i],0),steps=1)
        dx = result[0][0]
        dy = result[0][1]
        ndcz = result[0][2]
        heady = result[0][3]
        headp = result[0][4]
        skinr = result[0][5]
        sking = result[0][6]
        skinb = result[0][7]

        # reconstruct screen coordinates from fscan prediction and estimated deltas
        screenx = datas[i].pred[0] + dx
        screeny = datas[i].pred[1] + dy
        escreenx = datas[i].screen[0] - screenx
        escreeny = datas[i].screen[1] - screeny

        # convert to NDC
        ndcx = 2.0 * screenx / width - 1.0
        ndcy = 1.0 - 2.0 * screeny / height
        endcx = datas[i].ndc[0] - ndcx
        endcy = datas[i].ndc[1] - ndcy
        endcz = datas[i].ndc[2] - ndcz

        # reconstruct position of head
        z = (mzw - mww * ndcz) / (mwz * ndcz - mzz)
        homw = mwz * z + mww
        x = homw * ndcx / mxx
        y = homw * ndcy / myy
        ex = datas[i].pos[0] - x
        ey = datas[i].pos[1] - y
        ez = datas[i].pos[2] - z

        eheady = datas[i].head[0] - heady
        eheadp = datas[i].head[1] - headp

        eskinr = datas[i].skin_color[0] - skinr
        esking = datas[i].skin_color[1] - sking
        eskinb = datas[i].skin_color[2] - skinb

        errors.append((escreenx,escreeny,endcx,endcy,endcz,ex,ey,ez,eheady,eheadp,eskinr,esking,eskinb))

    avg,stddev = calculate_measurement_errors(errors)
    
    print('measurement errors:')
    print('    screen.x: {:5.3f} +/- {:5.3f} pixels @ {} wide'.format(avg[0],stddev[0],width))
    print('    screen.y: {:5.3f} +/- {:5.3f} pixels @ {} high'.format(avg[1],stddev[1],height))
    print('    ndc.x:    {:5.3f} +/- {:5.3f}'.format(avg[2],stddev[2]))
    print('    ndc.y:    {:5.3f} +/- {:5.3f}'.format(avg[3],stddev[3]))
    print('    ndc.z:    {:5.3f} +/- {:5.3f}'.format(avg[4],stddev[4]))
    print('    pos.x:    {:5.3f} +/- {:5.3f}'.format(avg[5],stddev[5]))
    print('    pos.y:    {:5.3f} +/- {:5.3f}'.format(avg[6],stddev[6]))
    print('    pos.z:    {:5.3f} +/- {:5.3f}'.format(avg[7],stddev[7]))
    print('    head.y:   {:5.3f} +/- {:5.3f}'.format(avg[8],stddev[8]))
    print('    head.p:   {:5.3f} +/- {:5.3f}'.format(avg[9],stddev[9]))
    print('    skin.r:   {:5.3f} +/- {:5.3f}'.format(avg[10],stddev[10]))
    print('    skin.g:   {:5.3f} +/- {:5.3f}'.format(avg[11],stddev[11]))
    print('    skin.b:   {:5.3f} +/- {:5.3f}'.format(avg[12],stddev[12]))
