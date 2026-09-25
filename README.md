# How to launch

This one purpose program will not launch without the `Config.toml`.

## Config.toml example for 1920 x 1080

```
window_name = "exaple"

allowed_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"

# pixels
# tracks the moment when to start the recognition cycle
start_pixel_x = 0
start_pixel_y = 0

# from what part of the full screen size should try to recognize words
roi_x = 0
roy_y = 0
roi_width = 0
roi_height = 0

char_width = 0
char_height = 0

How many pixels of the same color must be present in an area for it to be recognized as the beginning of a word
required_pixels_for_start = 30

How many pixels of a certain color should be in an area to define the boundaries of a word
required_pixels_for_borders = 10

# 0 - 255
start_pixel_red = 0
start_pixel_green = 0
start_pixel_blue = 0

start_char_r_max = 0
start_char_r_min = 0
start_char_g_max = 0
start_char_g_min = 0
start_char_b_max = 0
start_char_b_min = 0

bin_start_char_r_max = 0
bin_start_char_r_min = 0
bin_start_char_g_max = 0
bin_start_char_g_min = 0
bin_start_char_b_max = 0
bin_start_char_b_min = 0

char_white = 0
bin_char_white = 0

# secs
work_timer = 0

# millis
delay_before_word_appears = 0
delay_type_min = 0
delay_type_max = 0
delay_before_new_capture = 0
delay_defore_new_word_appears = 0
```