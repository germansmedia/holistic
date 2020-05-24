#!/usr/bin/python3
# -*- coding: utf-8 -*-

import sys
from math import *
import numpy as np
from PyQt5.QtWidgets import *
from PyQt5.QtGui import *
from PyQt5.QtCore import *
from PyQt5.QtOpenGL import *
from OpenGL.GL import *
from OpenGL.GL.shaders import *

OPENGL_MAJOR = 4
OPENGL_MINOR = 2

class Graphics(QGLWidget):

    def __init__(self):
        self.opengl_format = QGLFormat()
        self.opengl_format.setVersion(OPENGL_MAJOR,OPENGL_MINOR)
        self.opengl_format.setProfile(QGLFormat.CoreProfile)
        self.opengl_format.setSampleBuffers(True)
        self.opengl_format.setDepth(True)
        super(Graphics,self).__init__(self.opengl_format)
        self.makeCurrent()
