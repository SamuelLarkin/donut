// g++ donut.c && ./a.out
#include <cstring>
#include <math.h>       /* sin */
#include <stdio.h>      /* printf */
#include <unistd.h>

int main() {
   const unsigned int screen_width = 80;
   const unsigned int screen_height = 30;

   float A = 0, B = 0;
   float i, j;
   int k;
   float z[screen_width * screen_height];   // 1760
   char b[screen_width * screen_height];   // 1760

   printf("\x1b[2J");
   for(;;) {
      memset(b, 32, screen_width * screen_height * sizeof(char));
      memset(z, 0, screen_width * screen_height * sizeof(float));
      for(j=0; j < 6.28; j += 0.07) {
         for(i=0; i < 6.28; i += 0.02) {
            const float c = sin(i);
            const float d = cos(j);
            const float e = sin(A);
            const float f = sin(j);
            const float g = cos(A);
            const float h = d + 2;
            const float D = 1 / (c * h * e + f * g + 5);
            const float l = cos(i);
            const float m = cos(B);
            const float n = sin(B);
            const float t = c * h * g - f * e;
            const int x = 40 + 30 * D * (l * h * m - t * n);
            const int y= 12 + 15 * D * (l * h * n + t * m);
            const int o = x + screen_width * y;
            const int N = 8 * ((f * e - c * d * g) * m - c * d * e - f * g - l * d * n);
            if(screen_height > y && y > 0 && x > 0 && screen_width > x && D > z[o]) {
               z[o] = D;
               b[o] = ".,-~:;=!*#$@"[N > 0 ? N : 0];
            }
         }
      }
      printf("\x1b[H");
      for(k = 0; k < screen_width*screen_height; k++) {
         putchar(k % screen_width ? b[k] : 10);
         A += 0.00004;
         B += 0.00002;
      }
      usleep(30000);
   }
   return 0;
}
