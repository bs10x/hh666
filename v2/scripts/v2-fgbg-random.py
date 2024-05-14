import random

# Define the ANSI color codes for background and foreground
bg_colors = [40, 41, 42, 43, 44, 45, 46, 47, 100, 101, 102, 103, 104, 105, 106, 107]
fg_colors = [30, 31, 32, 33, 34, 35, 36, 37, 90, 91, 92, 93, 94, 95, 96, 97]

# Loop 500 times
for _ in range(17000):
    # Choose a random background color
    bg_color = random.choice(bg_colors)
    
    # Choose a random foreground color that is not the same as the background color #  if color % 10 != bg_color % 10 #
    fg_color = random.choice([color for color in fg_colors])
    
    # Choose a random ASCII character that is printable and not a space or a tab
    char = random.choice([chr(i) for i in range(33, 127) if i != 32 and i != 9])
    
    # Start the line with the color code
    line = f'\033[{fg_color};{bg_color}m{char}\033[0m'
    
    # Print the line
    print(line, end='')
