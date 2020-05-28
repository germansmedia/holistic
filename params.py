#!/usr/bin/python3
# -*- coding: utf-8 -*-

import sys
import yaml

class Params:

    def __init__(self,name):

        params = yaml.load(open(name,'r'))

        self.frame_width = params['frame_width']
        self.frame_height = params['frame_height']
        self.factor = params['factor']
        self.threshold = params['threshold']
        self.cutout_size = params['cutout_size']
        self.scan_filters = params['scan_filters']
        self.scan_modules = params['scan_modules']
        self.scan_rate = params['scan_rate']
        self.scan_batch_size = params['scan_batch_size']
        self.scan_epochs = params['scan_epochs']
        self.pose_filters = params['pose_filters']
        self.pose_modules = params['pose_modules']
        self.pose_rate = params['pose_rate']
        self.pose_batch_size = params['pose_batch_size']
        self.pose_epochs = params['pose_epochs']
