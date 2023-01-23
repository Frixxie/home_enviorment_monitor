SRC = collector.ino

.PHONY: compile upload

all: compile upload

compile: $(SRC)
	arduino-cli compile -b esp32:esp32:nodemcu-32s

upload: compile
	arduino-cli upload -b esp32:esp32:nodemcu-32s --port /dev/ttyUSB0
