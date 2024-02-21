# Syracuse

Print Syracuse suite elements, number of elements and maximum value.

## Installation

### Unix users (Linux and MacOSX)

Unix users may download and install latest *syracuse* release with command:

```bash
sh -c "$(curl https://sweetohm.net/dist/syracuse/install)"
```

If *curl* is not installed on you system, you might run:

```bash
sh -c "$(wget -O - https://sweetohm.net/dist/syracuse/install)"
```

**Note:** Some directories are protected, even as *root*, on **MacOSX** (since *El Capitan* release), thus you can't install *syracuse* in */usr/bin* for instance.

### Binary package

Otherwise, you can download latest binary archive at <https://github.com/c4s4/syracuse/releases>. Unzip the archive, put the binary of your platform somewhere in your *PATH* and rename it *syracuse*.

## Usage

To print Syracuse suite elements for given number type:

```bash
$ syracuse 9
9 28 14 7 22 11 34 17 52 26 13 40 20 10 5 16 8 4 2 1 (20) [52]
```

You can pass several numbers:

```bash
$ syracuse 9 10 11
9 28 14 7 22 11 34 17 52 26 13 40 20 10 5 16 8 4 2 1 (20) [52]
10 5 16 8 4 2 1 (7) [16]
11 34 17 52 26 13 40 20 10 5 16 8 4 2 1 (15) [52]
```

*Enjoy!*
