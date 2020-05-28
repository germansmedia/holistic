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
from scan import *
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

        if self.holistic.application.input_image:

            # calculate uniform scale
            xscale = self.width() / self.holistic.application.params.frame_width
            yscale = self.height() / self.holistic.application.params.frame_height
            if self.holistic.application.params.frame_height * xscale > self.height():
                scale = yscale
            else:
                scale = xscale

            # calculate resulting frame scale and offset
            xsize = scale * self.holistic.application.params.frame_width
            ysize = scale * self.holistic.application.params.frame_height
            xstart = 0.5 * (self.width() - xsize)
            ystart = 0.5 * (self.height() - ysize)

            # calculate multiplier for frame coordinates -> widget coordinates
            xpsize = xsize / self.holistic.application.params.frame_width
            ypsize = ysize / self.holistic.application.params.frame_height

            # draw frame
            painter.drawImage(QRect(xstart,ystart,xsize,ysize),self.holistic.application.input_image)

            if self.holistic.application.scan_image:

                # draw scan result cells
                print('drawing cells {},{} ({}x{})'.format(xstart,ystart,xsize,ysize))
                painter.setCompositionMode(QPainter.CompositionMode_Plus)
                painter.drawImage(QRect(xstart,ystart,xsize,ysize),self.holistic.application.scan_image)

                if self.holistic.application.face_params != None:

                    # draw blue rectangle of grabbed image
                    px = self.holistic.application.face_params[0]
                    py = self.holistic.application.face_params[1]
                    full = self.holistic.application.params.cutout_size
                    half = int((full - 1) / 2)
                    painter.fillRect(xstart + (px - half) * xpsize,ystart + (py - half) * ypsize,full * xpsize,full * ypsize,QColor(0,0,255))

                    # draw green dot at predicted screen coordinates
                    dx = self.holistic.application.face_params[2][0]
                    dy = self.holistic.application.face_params[2][1]
                    screenx = px + dx
                    screeny = py + dy
                    painter.fillRect(xstart + (screenx - 2) * xpsize,ystart + (screeny - 2) * ypsize,5 * xpsize,5 * ypsize,QColor(0,255,0))

                    # draw rectangle image in the corner
                    painter.setCompositionMode(QPainter.CompositionMode_Source)
                    painter.drawImage(0,0,self.holistic.application.pose_image)

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

        self.params = Params('params.yaml')

        self.graphics = Graphics()

        self.stop_process = False
        self.lastWindowClosed.connect(self.quit)

        if not TEST_IMAGE:
            self.camera = cv2.VideoCapture(0)
            self.camera.set(cv2.CAP_PROP_FRAME_WIDTH,640)
            self.camera.set(cv2.CAP_PROP_FRAME_HEIGHT,480)
            self.camera.set(cv2.CAP_PROP_FPS,60)

        self.input_image = None
        self.scan_image = None
        self.pose_image = None
        self.face_params = None

        self.scan_model = ScanModel(self.params,'scan.h5')
        self.pose_model = PoseModel(self.params,'pose.h5')
        self.holistic = Holistic(self)
        if not TEST_IMAGE:
            self.process_thread = threading.Thread(target=self.processThread,args=())
            self.process_thread.start()
        else:
            input_frame = cv2.resize(cv2.imread('test0/00004.bmp'),(self.params.frame_width,self.params.frame_height))
            self.process(input_frame)


    def quit(self):

        if not TEST_IMAGE:
            self.stop_process = True
            self.process_thread.join()

    def process(self,input_frame):

        # turn into QImage
        bgr_version = cv2.cvtColor(input_frame,cv2.COLOR_RGB2BGR)
        self.input_image = QImage(bgr_version.data,bgr_version.shape[1],bgr_version.shape[0],QImage.Format_RGB888)

        # run scan
        scan_input = np.multiply(input_frame.astype(np.float32),1.0 / 255.0)
        scan_output = self.scan_model.infer(scan_input)
        scan_frame = cv2.cvtColor(np.multiply(scan_output,255.0).astype(np.uint8),cv2.COLOR_GRAY2RGBA)

        # clear green & blue channels, and set alpha
        scan_frame[:,:,1] = 0
        scan_frame[:,:,2] = 0
        scan_frame[:,:,3] = 255

        # turn into QImage
        self.scan_image = QImage(scan_frame.data,scan_frame.shape[1],scan_frame.shape[0],QImage.Format_RGBA8888)

        # -- copied from scan.py (refactor into separate calls):

        # width and height of scan_output
        rwidth = int(math.floor(self.params.frame_width / self.params.factor))
        rheight = int(math.floor(self.params.frame_height / self.params.factor))

        # find the cell with the highest result
        px = -1
        py = -1
        highest = 0.0
        for y in range(0,rheight):
            for x in range(0,rwidth):
                if scan_output[y,x] > highest:
                    highest = scan_output[y,x]
                    px = x
                    py = y

        if highest > self.params.threshold:

            # adjust subcell accuracy
            if py > 0:
                u = scan_output[py - 1,px]
            else:
                u = 0.0
            if py < rheight - 1:
                d = scan_output[py + 1,px]
            else:
                d = 0.0
            if px > 0:
                l = scan_output[py,px - 1]
            else:
                l = 0.0
            if px < rwidth - 1:
                r = scan_output[py,px + 1]
            else:
                r = 0.0
            totalx = highest + l + r
            totaly = highest + u + d
            ax = (r - l) / totalx
            ay = (d - u) / totaly

            # add adjustment, and take center of the cell
            px = int((px + 0.5 + ax) * self.params.factor)
            py = int((py + 0.5 + ay) * self.params.factor)

            # create cutout
            pose_input = np.zeros((self.params.cutout_size,self.params.cutout_size,3),np.float32)
            half = int((self.params.cutout_size - 1) / 2)
            for y in range(-half,half + 1):
                cy = int(py + y)
                if (cy >= 0) and (cy < self.params.frame_height):
                    for x in range(-half,half + 1):
                        cx = int(px + x)
                        if (cx >= 0) and (cx < self.params.frame_width):
                            pose_input[half + y,half + x] = scan_input[cy,cx]

            # turn into QImage
            bgra_version = cv2.cvtColor(pose_input,cv2.COLOR_RGB2BGR)
            self.pose_image = QImage(bgra_version.data,bgra_version.shape[1],bgra_version.shape[0],QImage.Format_RGB888)

            # run pose
            pose_output = self.pose_model.infer(pose_input)
                
            # update the face parameter
            self.face_params = (px,py,pose_output)

        else:
            self.face_params = None

        # update the viewer
        self.holistic.viewer.update()

    def processThread(self):
        while not self.stop_process:
            ret,input_frame = self.camera.read()
            input_frame = cv2.resize(input_frame,(self.params.frame_width,self.params.frame_height))
            self.process(input_frame)

if __name__ == '__main__':

    #formats = getCameraFormats(1)
    #for fmt in formats:
    #    print('{}x{} @ {} ({})'.format(fmt[1],fmt[2],fmt[3],fmt[0]))

    sys.exit(Application(sys.argv).exec_())
