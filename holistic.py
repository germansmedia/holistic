#!/usr/bin/python3
# -*- coding: utf-8 -*-

import sys
import signal 
import threading
import os
import cv2
import v4l2
import fcntl
from pathlib import *
from math import *
from PyQt5.QtWidgets import *
from PyQt5.QtGui import *
from PyQt5.QtCore import *
import tensorflow as tf
from tensorflow.keras import Model
from graphics import *
from fscan import *
from pose import *

TEST_IMAGE = False

def getCameraFormats(number):
    name = '/dev/video{}'.format(number)
    formats = []
    try:
        with open(name,'r') as fd:
            fmt = v4l2.v4l2_fmtdesc()
            fmt.index = 0
            fmt.type = v4l2.V4L2_CAP_VIDEO_CAPTURE
            while fcntl.ioctl(fd,v4l2.VIDIOC_ENUM_FMT,fmt) == 0:
                pixelformat = '{:c}{:c}{:c}{:c}'.format(
                    fmt.pixelformat & 255,
                    (fmt.pixelformat >> 8) & 255,
                    (fmt.pixelformat >> 16) & 255,
                    fmt.pixelformat >> 24
                )
                frmsize = v4l2.v4l2_frmsizeenum()
                frmsize.index = 0
                frmsize.pixel_format = fmt.pixelformat
                while fcntl.ioctl(fd,v4l2.VIDIOC_ENUM_FRAMESIZES,frmsize) == 0:
                    if frmsize.type != v4l2.V4L2_FRMSIZE_TYPE_DISCRETE:
                        break
                    width = frmsize.discrete.width
                    height = frmsize.discrete.height
                    frmival = v4l2.v4l2_frmivalenum()
                    frmival.index = 0
                    frmival.pixel_format = fmt.pixelformat
                    frmival.width = width
                    frmival.height = height
                    try:
                        while fcntl.ioctl(fd,v4l2.VIDIOC_ENUM_FRAMEINTERVALS,frmival) == 0:
                            if frmival.type != v4l2.V4L2_FRMIVAL_TYPE_DISCRETE:
                                break
                            fps = int(frmival.discrete.denominator / frmival.discrete.numerator)
                            formats.append((pixelformat,width,height,fps))
                            frmival.index += 1
                    except:
                        None
                    frmsize.index += 1
                fmt.index += 1
    except IOError as e:
        None
    return formats

class ViewerWidget(QWidget):

    def __init__(self,holistic,parent=None):
        super(ViewerWidget,self).__init__(parent)
        self.holistic = holistic

    def paintEvent(self,event):
        painter = QPainter()
        painter.begin(self)
        factor = self.holistic.application.fscan_model.params['factor']
        if self.holistic.application.input_image:
            xscale = self.width() / self.holistic.application.input_image.width()
            yscale = self.height() / self.holistic.application.input_image.height()
            if self.holistic.application.input_image.height() * xscale > self.height():
                scale = yscale
            else:
                scale = xscale
            xsize = scale * self.holistic.application.input_image.width()
            ysize = scale * self.holistic.application.input_image.height()
            xstart = 0.5 * (self.width() - xsize)
            ystart = 0.5 * (self.height() - ysize)
            xpsize = xsize / self.holistic.application.input_image.width()
            ypsize = ysize / self.holistic.application.input_image.height()
            painter.drawImage(QRect(xstart,ystart,xsize,ysize),self.holistic.application.input_image)
            painter.setCompositionMode(QPainter.CompositionMode_Plus)
            if self.holistic.application.fscan_image:
                painter.drawImage(QRect(xstart,ystart,xsize,ysize),self.holistic.application.fscan_image)
                if self.holistic.application.face_params != None:
                    px = self.holistic.application.face_params[0]
                    py = self.holistic.application.face_params[1]
                    painter.fillRect(xstart + (px - 65) * xpsize,ystart + (py - 65) * ypsize,129 * xpsize,129 * ypsize,QColor(0,0,255))
                    painter.setCompositionMode(QPainter.CompositionMode_SourceOver)
                    painter.drawImage(0,0,self.holistic.application.pose_image)
                    dx = self.holistic.application.face_params[2][0]
                    dy = self.holistic.application.face_params[2][1]
                    ndcz = self.holistic.application.face_params[2][2]
                    heady = self.holistic.application.face_params[2][3]
                    headp = self.holistic.application.face_params[2][4]
                    screenx = self.holistic.application.face_params[0] + dx
                    screeny = self.holistic.application.face_params[1] + dy
                    painter.fillRect(xstart + (screenx - 2) * xpsize,ystart + (screeny - 2) * ypsize,5 * xpsize,5 * ypsize,QColor(0,255,0))
        painter.end()

class Holistic(QMainWindow):

    def __init__(self,application):
        super(Holistic,self).__init__()
        self.application = application
        self.viewer = ViewerWidget(self)
        self.setCentralWidget(self.viewer)
        self.initActions()
        self.initMenuBar()
        self.setWindowTitle('Holistic Scanner')
        self.setWindowIcon(QIcon('icons/holistic.png'))
        self.setGeometry(100,100,1024,768)
        self.show()

    def initActions(self):
        self.actions_exit = QAction(QIcon('icons/quit.png'), 'Exit', self)
        self.actions_exit.setShortcut('Ctrl+X')
        self.actions_exit.setStatusTip('Exit')
        self.actions_exit.triggered.connect(self.close)

    def initMenuBar(self):
        menubar = self.menuBar()
        self.file_menu = menubar.addMenu('&File')
        self.file_menu.addAction(self.actions_exit)
        self.camera_menu = menubar.addMenu('&Camera')

    def closeEvent(self,event):
        event.accept()

class Application(QApplication):

    def __init__(self,argv):
        super(QApplication,self).__init__(argv)
        self.stop_process = False
        self.lastWindowClosed.connect(self.quit)
        self.graphics = Graphics()
        if not TEST_IMAGE:
            self.camera = cv2.VideoCapture(0)
            self.camera.set(cv2.CAP_PROP_FRAME_WIDTH,640)
            self.camera.set(cv2.CAP_PROP_FRAME_HEIGHT,480)
            self.camera.set(cv2.CAP_PROP_FPS,60)
        self.input_image = None
        self.fscan_image = None
        self.face_params = None
        self.fscan_model = ModelFSCAN()
        self.pose_model = ModelPOSE()
        self.holistic = Holistic(self)
        if not TEST_IMAGE:
            self.process_thread = threading.Thread(target=self.processThread,args=())
            self.process_thread.start()
        else:
            self.process(cv2.cvtColor(cv2.imread('test0/00250.bmp'),cv2.COLOR_BGR2RGB))

    def quit(self):
        if not TEST_IMAGE:
            self.stop_process = True
            self.process_thread.join()

    def process(self,input_frame):
            # turn into QImage
            self.input_image = QImage(input_frame.data,input_frame.shape[1],input_frame.shape[0],QImage.Format_RGB888)

            # run fscan
            fscan_input = np.multiply(input_frame.astype(np.float32),1.0 / 255.0)
            fscan_output = self.fscan_model(fscan_input)
            fscan_frame = cv2.cvtColor(np.multiply(fscan_output,255.0).astype(np.uint8),cv2.COLOR_GRAY2RGBA)

            # clear green & blue channels, and set alpha
            fscan_frame[:,:,1] = 0
            fscan_frame[:,:,2] = 0
            fscan_frame[:,:,3] = 255

            # turn into QImage
            self.fscan_image = QImage(fscan_frame.data,fscan_frame.shape[1],fscan_frame.shape[0],QImage.Format_RGBA8888)

            # find peak
            width = self.fscan_model.params['width']
            height = self.fscan_model.params['height']
            factor = self.fscan_model.params['factor']
            px = -1
            py = -1
            highest = 0.0
            threshold = 0.1
            for y in range(0,int(height / factor)):
                for x in range(0,int(width / factor)):
                    if fscan_output[y][x] > highest:
                        highest = fscan_output[y][x]
                        px = x
                        py = y

            # create cutout
            if highest > threshold:
                px *= factor
                py *= factor

                pose_input = np.zeros((129,129,3),np.float32)
                for y in range(-64,65):
                    cy = int(py + y)
                    if (cy >= 0) and (cy < height):
                        for x in range(-64,65):
                            cx = int(px + x)
                            if (cx >= 0) and (cx < width):
                                pose_input[y + 64,x + 64] = fscan_input[cy,cx]

                # turn into QImage
                self.pose_image = QImage(pose_input.data,pose_input.shape[1],pose_input.shape[0],QImage.Format_RGB888)

                # run pose
                pose_output = self.pose_model(pose_input)
                
                # update the data
                self.face_params = (px,py,pose_output)

            else:
                self.face_params = None

            # update the viewer
            self.holistic.viewer.update()

    def processThread(self):
        while not self.stop_process:
            ret,input_frame = self.camera.read()
            input_frame = cv2.resize(cv2.cvtColor(input_frame,cv2.COLOR_BGR2RGB),(self.fscan_model.params['width'],self.fscan_model.params['height']))
            self.process(input_frame)

if __name__ == '__main__':

    #formats = getCameraFormats(1)
    #for fmt in formats:
    #    print('{}x{} @ {} ({})'.format(fmt[1],fmt[2],fmt[3],fmt[0]))

    sys.exit(Application(sys.argv).exec_())
