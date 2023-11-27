import cv2 as cv

vid = cv.VideoCapture(0)

while True:
    _, frame = vid.read() # This variable is merged to cprogram.mg.c
    # cv.imshow('frame', frame)
    if cv.waitKey(1) & 0xFF == ord('q'):
        break
