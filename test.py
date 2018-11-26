#!/usr/bin/env python
from datetime import datetime
from datetime import  date, time
import pybakbuster

def doit():
    packages = "/Users/jonathangerber/src/rust/pybakbuster/examples/packages.xml"
    stack = "/Users/jonathangerber/src/rust/pybakbuster/examples/bak/packages.xml/packages.xml_swinstall_stack"
    print "reading swinstall stack to determine appropriate versions"
    with  open(stack) as fh:
        print fh.read()
        print
    now = datetime.now().ctime()
    print "retrieving appropriate packages.xml file @ {}".format(now)
    print pybakbuster.get_file_on(packages, now)

    # 20161213-093146
    d = date(2017,12,23)
    t = time(9,31,46)

    now = datetime.combine(d,t).ctime()
    print "retrieving appropriate packages.xml file @ {}".format(now)
    print pybakbuster.get_file_on(packages, now)

if __name__ == '__main__':
    doit()