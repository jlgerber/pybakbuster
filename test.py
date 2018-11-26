#!/usr/bin/env python
import datetime
import pybakbuster

def doit():
    now = datetime.datetime.now()
    print pybakbuster.get_file_on("/Users/jonathangerber/src/rust/bakbuster/examples/packages.xml_swinstall_stack", now.ctime())

if __name__ == '__main__':
    doit()