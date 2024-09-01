pragma circom  2.1.8;

template red_channel(size) {

    signal input orig_pixels[size];
    signal input new_pixels[size];
    signal output out;
    signal mid[size];

    var sum_check = 0; 

    for (var i = 0; i < size; i++) {
        mid[i] <-- (orig_pixels[i] & 0xff0000ff);
        mid[i] === new_pixels[i];
        sum_check += mid[i] - new_pixels[i];
    }

    out <== sum_check;
    0 === out;

}

component main = red_channel(32*32);
