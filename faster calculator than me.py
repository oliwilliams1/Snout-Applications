import math
import glm

theta = 90

def degrees_to_vector(theta) -> glm.vec2:
    return glm.vec2(math.cos(math.radians(theta)), math.sin(math.radians(theta)))

string = "clip-path: polygon(100% 0%, 50% 0%,"
while theta >= 45:
    vec = degrees_to_vector(theta)
    vec = glm.normalize(vec)
    coords = glm.vec2(vec * 30)
    
    theta -= 7.5

    string += f'{coords.x + 50}% {50 - coords.y}%'
    string += ', '

string = string[:-2]
string += ");"
print(string)