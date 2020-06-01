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
import pyrealsense2 as rs
from graphics import *
import scan
import pose
from params import *

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

        if self.holistic.application.input_image_rgb != None:

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
            #painter.drawImage(QRect(xstart,ystart,xsize,ysize),self.holistic.application.input_image_rgb)
            painter.drawImage(QRect(xstart,ystart,xsize,ysize),self.holistic.application.input_image_a)

            if self.holistic.application.scan_image != None:

                # draw scan result cells
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
            config = rs.config()
            config.enable_stream(rs.stream.color,0,640,480,rs.format.bgr8,60)
            config.enable_stream(rs.stream.depth,0,640,480,rs.format.z16,60)
            self.pipe = rs.pipeline()
            self.pipe.start(config)
            self.align = rs.align(rs.stream.color)

        self.input_image_rgb = None
        self.input_image_depth = None
        self.scan_image = None
        self.pose_image = None
        self.face_params = None

        self.scan_model = scan.create(self.params.frame_width,self.params.frame_height,self.params.factor,self.params.scan_filters,self.params.scan_modules,self.params.scan_rate)
        self.scan_model.load_weights('scan.h5')
        self.pose_model = pose.create(self.params.cutout_size,self.params.pose_filters,self.params.pose_rate)
        self.pose_model.load_weights('pose.h5')
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

    def process(self,input_frame_rgb,input_frame_a):

        # turn into QImage
        bgr_version = cv2.cvtColor(input_frame_rgb,cv2.COLOR_RGB2BGR)
        self.input_image_rgb = QImage(bgr_version.data,bgr_version.shape[1],bgr_version.shape[0],QImage.Format_RGB888)
        self.input_image_a = QImage(input_frame_a.data,input_frame_a.shape[1],input_frame_a.shape[0],QImage.Format_Grayscale8)

        # run scan
        combined = np.dstack((input_frame_rgb[:,:,0],input_frame_rgb[:,:,1],input_frame_rgb[:,:,2],input_frame_a))
        scan_input = np.multiply(combined.astype(np.float32),1.0 / 255.0)
        scan_output = scan.infer(self.scan_model,scan_input)

        # turn into red QImage
        scan_frame = cv2.cvtColor(scan.float2rgb(scan_output),cv2.COLOR_GRAY2RGBA)
        scan_frame[:,:,1] = 0
        scan_frame[:,:,2] = 0
        scan_frame[:,:,3] = 255
        self.scan_image = QImage(scan_frame.data,scan_frame.shape[1],scan_frame.shape[0],QImage.Format_RGBA8888)

        # find where the face is
        result = scan.find(scan_input,scan_output,self.params.frame_width,self.params.frame_height,self.params.factor,self.params.threshold)

        if result != None:

            # create cutout
            pose_input = scan.generate_cutout(result[0],result[1],scan_input,self.params.frame_width,self.params.frame_height,self.params.cutout_size)

            # turn into QImage
            bgra_version = cv2.cvtColor(pose_input,cv2.COLOR_RGB2BGR)
            self.pose_image = QImage(bgra_version.data,bgra_version.shape[1],bgra_version.shape[0],QImage.Format_RGB888)

            # run pose
            pose_output = pose.infer(self.pose_model,pose_input)
                
            # update the face parameter
            self.face_params = (result[0],result[1],pose_output)

        else:
            self.face_params = None

        # update the viewer
        self.holistic.viewer.update()

    def processThread(self):
        while not self.stop_process:
            frames = self.pipe.wait_for_frames()
            frames = self.align.process(frames)
            input_frame_rgb = np.asanyarray(frames.get_color_frame().data)
            input_frame_a = np.asanyarray(frames.get_depth_frame().data)
            #print('rgb shape = {}'.format(input_frame_rgb.shape))
            input_frame_a = np.clip(np.divide(input_frame_a,32.0),0.0,255.0).astype(np.uint8)
            input_frame_rgb = cv2.resize(input_frame_rgb,(self.params.frame_width,self.params.frame_height))
            input_frame_a = cv2.resize(input_frame_a,(self.params.frame_width,self.params.frame_height))
            self.process(input_frame_rgb,input_frame_a)

if __name__ == '__main__':

    #formats = getCameraFormats(1)
    #for fmt in formats:
    #    print('{}x{} @ {} ({})'.format(fmt[1],fmt[2],fmt[3],fmt[0]))

    sys.exit(Application(sys.argv).exec_())
