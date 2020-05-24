import tensorflow as tf
from tensorflow.keras import datasets,layers,models,Model
import csv
import cv2
import numpy as np
import math
import yaml
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
    params = yaml.load(open('fscan.yaml','r'))
    inputs,outputs,datas = modeldef.load_data('../../test0/',params)
    model = modeldef.make_model(params)
    model.load_weights('fscan.h5')
    width = params['width']
    height = params['height']
    factor = params['factor']
    errors = []
    nerrors = []
    threshold = 0.3
    for i in range(0,len(inputs)):
        result = model.predict(tf.expand_dims(inputs[i],0),steps=1)
        rx = datas[i].screen[0] / factor
        ry = datas[i].screen[1] / factor
        px = -1
        py = -1
        highest = 0.0
        for y in range(0,int(height / factor)):
            for x in range(0,int(width / factor)):
                if result[0][y][x][0] > highest:
                    highest = result[0][y][x][0]
                    px = x
                    py = y
        if highest > threshold:

            # subpixel adjustment
            r00 = highest
            if px > 0:
                r0n = result[0][py][px - 1][0]
            else:
                r0n = 0.0
            if px < int(height / factor) - 1:
                r0p = result[0][py][px + 1][0]
            else:
                r0p = 0.0
            if py > 0:
                rn0 = result[0][py - 1][px][0]
            else:
                rn0 = 0.0
            if py < int(height / factor) - 1:
                rp0 = result[0][py + 1][px][0]
            else:
                rp0 = 0.0

            a = 0.5 * r0p + 0.5 * r0n - r00
            b = 0.5 * r0p - 0.5 * r0n
            npx = px - b / (2 * a)

            a = 0.5 * rp0 + 0.5 * rn0 - r00
            b = 0.5 * r00 - 0.5 * rn0
            npy = py - b / (2 * a)

            print('X = {:5.3f} {:5.3f} {:5.3f}, was {:5.3f}, now {:5.3f}'.format(r0n,r00,r0p,px,npx))
            print('Y = {:5.3f} {:5.3f} {:5.3f}, was {:5.3f}, now {:5.3f}'.format(rn0,r00,rp0,py,npy))

            ex = rx - px
            ey = ry - py
            errors.append((ex,ey))
            nex = rx - npx
            ney = ry - npy
            nerrors.append((nex,ney))
            print('image {}: real {:5.3f},{:5.3f}, predicted {:5.3f},{:5.3f}, error {:5.3f},{:5.3f}'.format(i,rx,ry,npx,npy,nex,ney))

        else:
            print('image {}: not found (max {:5.3f}).'.format(i,highest))

    avg,stddev = calculate_measurement_errors(errors)
    navg,nstddev = calculate_measurement_errors(nerrors)
    print('with rounding:')
    print('    X: {:5.3f} +/- {:5.3f} (+/- {:5.3f} in source)'.format(avg[0],stddev[0],stddev[0] * factor))
    print('    Y: {:5.3f} +/- {:5.3f} (+/- {:5.3f} in source)'.format(avg[1],stddev[1],stddev[1] * factor))
    print('with quadratic approximation:')
    print('    X: {:5.3f} +/- {:5.3f} (+/- {:5.3f} in source)'.format(navg[0],nstddev[0],nstddev[0] * factor))
    print('    Y: {:5.3f} +/- {:5.3f} (+/- {:5.3f} in source)'.format(navg[1],nstddev[1],nstddev[1] * factor))
