import tensorflow as tf
from tensorflow.keras import datasets,layers,models,Model
import os
import csv
import cv2
import numpy as np
import math
import shutil
import yaml
import modeldef

if __name__ == "__main__":
    params = yaml.load(open('fscan.yaml','r'))
    for chunk in ['data','test']:
        inputs,outputs,datas = modeldef.load_data('../../{}0/'.format(chunk),params)
        model = modeldef.make_model(params)
        model.load_weights('fscan.h5')
        width = params['width']
        height = params['height']
        factor = params['factor']
        threshold = 0.3
        if os.path.exists('../../{}1'.format(chunk)):
            shutil.rmtree('../../{}1'.format(chunk))
        os.mkdir('../../{}1'.format(chunk))
        with open('../../{}1/files.csv'.format(chunk),'w',newline='') as file:
            writer = csv.writer(file)
            n = 0
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
                    ex = rx - px
                    ey = ry - py
                    print('image {}: real {:5.3f},{:5.3f}, predicted {:5.3f},{:5.3f}, error {:5.3f},{:5.3f}'.format(i,rx,ry,px,py,ex,ey))

                    # create cutout of the found face
                    image = np.zeros((129,129,3),np.float32)
                    for y in range(-64,65):
                        cy = int(py * factor + y)
                        if (cy >= 0) and (cy < height):
                            for x in range(-64,65):
                                cx = int(px * factor + x)
                                if (cx >= 0) and (cx < width):
                                    image[y + 64,x + 64] = inputs[i][cy,cx]
                    image = np.multiply(image,255.0)
                    image = image.astype(np.uint8)
                    image_name = '{:05}.bmp'.format(n)
                    cv2.imwrite('../../{}1/'.format(chunk) + image_name,image)

                    # save corresponding data
                    writer.writerow([
                        image_name,
                        int(px * factor),int(py * factor),
                        datas[i].screen[0],datas[i].screen[1],
                        datas[i].pos[0],datas[i].pos[1],datas[i].pos[2],
                        datas[i].head[0],datas[i].head[1],
                        datas[i].ndc[0],datas[i].ndc[1],datas[i].ndc[2],
                        datas[i].light[0],datas[i].light[1],
                        datas[i].background_color[0],datas[i].background_color[1],datas[i].background_color[2],
                        datas[i].light_color[0],datas[i].light_color[1],datas[i].light_color[2],
                        datas[i].ambient_color[0],datas[i].ambient_color[1],datas[i].ambient_color[2],
                        datas[i].skin_color[0],datas[i].skin_color[1],datas[i].skin_color[2]
                    ])
                    n += 1
        print('generated {} valid face croppings for {} set'.format(n,chunk))
