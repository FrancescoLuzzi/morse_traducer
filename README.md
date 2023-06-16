# rust_playground
 
## wav refactoring

### Instument struc

function sin_wave(t,freq)

abstracts:
- layering of multiple frequencies to riproduce the correct sound
- amplitude behaviour over time [link](https://youtu.be/OSCzKOqtgcA?t=841)
- playing one or multiple notes
- (later)possibility to add modulation to the wave
- (later)possibility to add multiple modifiers to the wave as functions
- (later)possibility to add multiple filters to the wave as functions

### Note Enum

abstract the frequency of the note

### Generator

- Internal relative time where you can play different instruments
- Streamline notes and instruments (sequential, for baseline or repetitive harmonies)
- Different outputs:
  - raw stream of bytes
  - wav file