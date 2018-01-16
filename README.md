## Dist2Steps: fitness convertion helper tool

Dist2Steps is to help daily steps tracking in Google Fit or similar services
for users that are used to threadmill walking.

It converts the distance to steps (surprisingly), concidering one's height and,
consequently, the step lenght.

#### Usage:
For example let's take a 170cm high person, that walked 4.7 kilometers
```
# dist2steps 170 4.7
```
```
distance:    4.70  kilometers
step length: 70.38 centimeters
steps:       6678
```

#### Suported units:
##### Height
Centimeters
##### Distance
Kilometers