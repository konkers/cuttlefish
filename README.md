# Cuttlefish

![A drawing of a cuttlefish](img/cuttlefish.png)

An [RP2040](https://www.raspberrypi.com/documentation/microcontrollers/rp2040.html)
based controller for the [LumenPnP Feeder](https://github.com/opulo-inc/feeder).

## Cloning

This repo uses submodule to pull in external dependencies and needs to be recursivly cloned:

```bash
git clone --recurse-submodules https://github.com/konkers/cuttlefish 
```

## PCB

The main pcb is designed in [KiCad](kicad.org).  Becase I use the experimental
HTTP Library support, a nightly version of KiCad is needed.

The [schematic](https://kicanvas.org/?github=https%3A%2F%2Fgithub.com%2Fkonkers%2Fcuttlefish%2Fblob%2Fmain%2Fpcb%2Fmain%2Fcuttlefish-main.kicad_sch) and
[layout](https://kicanvas.org/?github=https%3A%2F%2Fgithub.com%2Fkonkers%2Fcuttlefish%2Fblob%2Fmain%2Fpcb%2Fmain%2Fcuttlefish-main.kicad_pcb)
can be viewed through KiCanvas.

### Front

![front of the cuttlefish pcb](img/main_front.png) 

### Back

![front of the cuttlefish pcb](img/main_back.png)

## Firmware

Firmware has not been written yet.  Check back for updates.
