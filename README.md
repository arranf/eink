# ePaper Display

## Hardware

Designed for the [ePD Waveshare 7.5inch V2](https://www.waveshare.com/7.5inch-e-paper-hat.htm) and a Raspberry Pi.

Requires the SPI and GPIO to be activated using `sudo raspi-config`. Requires sudo because of some sysfs_gpio permission problems and follow-up timing problems. See [here](https://github.com/rust-embedded/rust-sysfs-gpio/issues/5) for further details.

## Software

By default, randomly selects a `.txt` file, assumed to be only binary 1s and 0s, from the root directory to display and renders it.

Can be invoked with `eink ./path/to/file.txt` to display the chosen image.

## Scripts

* Build and sync to the Raspberry Pi using cross: `scripts/pi-build.sh`.
* Mount the file system: `scripts/pi-mount.example.sh`
