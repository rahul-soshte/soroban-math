# Observations / Thoughts

1. If you want to build a I256 Library you need to have a I512 type so that the intermediate calculations are more precise. So currently I am using I256 type for the i128 type intermediate calculations. So if I try building I512 bit, then it is necessary as fact to have basic arithmetic support for I1024.