
#[derive(Clone, Copy, Default, Debug)]
pub struct Color{
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

pub fn mix(a:Color, b:Color, f:f32)->Color{
    let nf = 1.0 - f;
    return Color{
        r: nf * a.r + f * b.r,
        g: nf * a.g + f * b.g,
        b: nf * a.b + f * b.b,
        a: nf * a.a + f * b.a,
    }
}

pub fn color256(r:i32, g:i32, b:i32)->Color{
    Color{r:r as f32 / 255., g:g as f32 / 255., b:b as f32 / 255., a:1.}
}

pub fn color256a(r:i32, g:i32, b:i32, a:i32)->Color{
    Color{r:r as f32 / 255., g:g as f32 / 255., b:b as f32 / 255., a:a as f32 / 255.}
}

pub fn color(name:&str)->Color{
    match name.to_lowercase().as_ref(){
        "aliceblue"=>Color{r:0.9411764705882353,g:0.9725490196078431,b:1.0,a:1.0},
        "antiquewhite"=>Color{r:0.9803921568627451,g:0.9215686274509803,b:0.8431372549019608,a:1.0},
        "aqua"=>Color{r:0.0,g:1.0,b:1.0,a:1.0},
        "aquamarine"=>Color{r:0.4980392156862745,g:1.0,b:0.8313725490196079,a:1.0},
        "azure"=>Color{r:0.9411764705882353,g:1.0,b:1.0,a:1.0},
        "beige"=>Color{r:0.9607843137254902,g:0.9607843137254902,b:0.8627450980392157,a:1.0},
        "bisque"=>Color{r:1.0,g:0.8941176470588236,b:0.7686274509803922,a:1.0},
        "black"=>Color{r:0.0,g:0.0,b:0.0,a:1.0},
        "blanchedalmond"=>Color{r:1.0,g:0.9215686274509803,b:0.803921568627451,a:1.0},
        "blue"=>Color{r:0.0,g:0.0,b:1.0,a:1.0},
        "blueviolet"=>Color{r:0.5411764705882353,g:0.16862745098039217,b:0.8862745098039215,a:1.0},
        "brown"=>Color{r:0.6470588235294118,g:0.16470588235294117,b:0.16470588235294117,a:1.0},
        "burlywood"=>Color{r:0.8705882352941177,g:0.7215686274509804,b:0.5294117647058824,a:1.0},
        "cadetblue"=>Color{r:0.37254901960784315,g:0.6196078431372549,b:0.6274509803921569,a:1.0},
        "chartreuse"=>Color{r:0.4980392156862745,g:1.0,b:0.0,a:1.0},
        "chocolate"=>Color{r:0.8235294117647058,g:0.4117647058823529,b:0.11764705882352941,a:1.0},
        "coral"=>Color{r:1.0,g:0.4980392156862745,b:0.3137254901960784,a:1.0},
        "cornflower"=>Color{r:0.39215686274509803,g:0.5843137254901961,b:0.9294117647058824,a:1.0},
        "cornsilk"=>Color{r:1.0,g:0.9725490196078431,b:0.8627450980392157,a:1.0},
        "crimson"=>Color{r:0.8627450980392157,g:0.0784313725490196,b:0.23529411764705882,a:1.0},
        "cyan"=>Color{r:0.0,g:1.0,b:1.0,a:1.0},
        "darkblue"=>Color{r:0.0,g:0.0,b:0.5450980392156862,a:1.0},
        "darkcyan"=>Color{r:0.0,g:0.5450980392156862,b:0.5450980392156862,a:1.0},
        "darkgoldenrod"=>Color{r:0.7215686274509804,g:0.5254901960784314,b:0.043137254901960784,a:1.0},
        "darkgray"=>Color{r:0.6627450980392157,g:0.6627450980392157,b:0.6627450980392157,a:1.0},
        "darkgreen"=>Color{r:0.0,g:0.39215686274509803,b:0.0,a:1.0},
        "darkkhaki"=>Color{r:0.7411764705882353,g:0.7176470588235294,b:0.4196078431372549,a:1.0},
        "darkmagenta"=>Color{r:0.5450980392156862,g:0.0,b:0.5450980392156862,a:1.0},
        "darkolivegreen"=>Color{r:0.3333333333333333,g:0.4196078431372549,b:0.1843137254901961,a:1.0},
        "darkorange"=>Color{r:1.0,g:0.5490196078431373,b:0.0,a:1.0},
        "darkorchid"=>Color{r:0.6,g:0.19607843137254902,b:0.8,a:1.0},
        "darkred"=>Color{r:0.5450980392156862,g:0.0,b:0.0,a:1.0},
        "darksalmon"=>Color{r:0.9137254901960784,g:0.5882352941176471,b:0.47843137254901963,a:1.0},
        "darkseagreen"=>Color{r:0.5607843137254902,g:0.7372549019607844,b:0.5607843137254902,a:1.0},
        "darkslateblue"=>Color{r:0.2823529411764706,g:0.23921568627450981,b:0.5450980392156862,a:1.0},
        "darkslategray"=>Color{r:0.1843137254901961,g:0.30980392156862746,b:0.30980392156862746,a:1.0},
        "darkturquoise"=>Color{r:0.0,g:0.807843137254902,b:0.8196078431372549,a:1.0},
        "darkviolet"=>Color{r:0.5803921568627451,g:0.0,b:0.8274509803921568,a:1.0},
        "deeppink"=>Color{r:1.0,g:0.0784313725490196,b:0.5764705882352941,a:1.0},
        "deepskyblue"=>Color{r:0.0,g:0.7490196078431373,b:1.0,a:1.0},
        "dimgray"=>Color{r:0.4117647058823529,g:0.4117647058823529,b:0.4117647058823529,a:1.0},
        "dodgerblue"=>Color{r:0.11764705882352941,g:0.5647058823529412,b:1.0,a:1.0},
        "firebrick"=>Color{r:0.6980392156862745,g:0.13333333333333333,b:0.13333333333333333,a:1.0},
        "floralwhite"=>Color{r:1.0,g:0.9803921568627451,b:0.9411764705882353,a:1.0},
        "forestgreen"=>Color{r:0.13333333333333333,g:0.5450980392156862,b:0.13333333333333333,a:1.0},
        "fuchsia"=>Color{r:1.0,g:0.0,b:1.0,a:1.0},
        "gainsboro"=>Color{r:0.8627450980392157,g:0.8627450980392157,b:0.8627450980392157,a:1.0},
        "ghostwhite"=>Color{r:0.9725490196078431,g:0.9725490196078431,b:1.0,a:1.0},
        "gold"=>Color{r:1.0,g:0.8431372549019608,b:0.0,a:1.0},
        "goldenrod"=>Color{r:0.8549019607843137,g:0.6470588235294118,b:0.12549019607843137,a:1.0},
        "gray"=>Color{r:0.7450980392156863,g:0.7450980392156863,b:0.7450980392156863,a:1.0},
        "webgray"=>Color{r:0.5019607843137255,g:0.5019607843137255,b:0.5019607843137255,a:1.0},
        "green"=>Color{r:0.0,g:1.0,b:0.0,a:1.0},
        "webgreen"=>Color{r:0.0,g:0.5019607843137255,b:0.0,a:1.0},
        "greenyellow"=>Color{r:0.6784313725490196,g:1.0,b:0.1843137254901961,a:1.0},
        "honeydew"=>Color{r:0.9411764705882353,g:1.0,b:0.9411764705882353,a:1.0},
        "hotpink"=>Color{r:1.0,g:0.4117647058823529,b:0.7058823529411765,a:1.0},
        "indianred"=>Color{r:0.803921568627451,g:0.3607843137254902,b:0.3607843137254902,a:1.0},
        "indigo"=>Color{r:0.29411764705882354,g:0.0,b:0.5098039215686274,a:1.0},
        "ivory"=>Color{r:1.0,g:1.0,b:0.9411764705882353,a:1.0},
        "khaki"=>Color{r:0.9411764705882353,g:0.9019607843137255,b:0.5490196078431373,a:1.0},
        "lavender"=>Color{r:0.9019607843137255,g:0.9019607843137255,b:0.9803921568627451,a:1.0},
        "lavenderblush"=>Color{r:1.0,g:0.9411764705882353,b:0.9607843137254902,a:1.0},
        "lawngreen"=>Color{r:0.48627450980392156,g:0.9882352941176471,b:0.0,a:1.0},
        "lemonchiffon"=>Color{r:1.0,g:0.9803921568627451,b:0.803921568627451,a:1.0},
        "lightblue"=>Color{r:0.6784313725490196,g:0.8470588235294118,b:0.9019607843137255,a:1.0},
        "lightcoral"=>Color{r:0.9411764705882353,g:0.5019607843137255,b:0.5019607843137255,a:1.0},
        "lightcyan"=>Color{r:0.8784313725490196,g:1.0,b:1.0,a:1.0},
        "lightgoldenrod"=>Color{r:0.9803921568627451,g:0.9803921568627451,b:0.8235294117647058,a:1.0},
        "lightgray"=>Color{r:0.8274509803921568,g:0.8274509803921568,b:0.8274509803921568,a:1.0},
        "lightgreen"=>Color{r:0.5647058823529412,g:0.9333333333333333,b:0.5647058823529412,a:1.0},
        "lightpink"=>Color{r:1.0,g:0.7137254901960784,b:0.7568627450980392,a:1.0},
        "lightsalmon"=>Color{r:1.0,g:0.6274509803921569,b:0.47843137254901963,a:1.0},
        "lightseagreen"=>Color{r:0.12549019607843137,g:0.6980392156862745,b:0.6666666666666666,a:1.0},
        "lightskyblue"=>Color{r:0.5294117647058824,g:0.807843137254902,b:0.9803921568627451,a:1.0},
        "lightslategray"=>Color{r:0.4666666666666667,g:0.5333333333333333,b:0.6,a:1.0},
        "lightsteelblue"=>Color{r:0.6901960784313725,g:0.7686274509803922,b:0.8705882352941177,a:1.0},
        "lightyellow"=>Color{r:1.0,g:1.0,b:0.8784313725490196,a:1.0},
        "lime"=>Color{r:0.0,g:1.0,b:0.0,a:1.0},
        "limegreen"=>Color{r:0.19607843137254902,g:0.803921568627451,b:0.19607843137254902,a:1.0},
        "linen"=>Color{r:0.9803921568627451,g:0.9411764705882353,b:0.9019607843137255,a:1.0},
        "magenta"=>Color{r:1.0,g:0.0,b:1.0,a:1.0},
        "maroon"=>Color{r:0.6901960784313725,g:0.18823529411764706,b:0.3764705882352941,a:1.0},
        "webmaroon"=>Color{r:0.4980392156862745,g:0.0,b:0.0,a:1.0},
        "mediumaquamarine"=>Color{r:0.4,g:0.803921568627451,b:0.6666666666666666,a:1.0},
        "mediumblue"=>Color{r:0.0,g:0.0,b:0.803921568627451,a:1.0},
        "mediumorchid"=>Color{r:0.7294117647058823,g:0.3333333333333333,b:0.8274509803921568,a:1.0},
        "mediumpurple"=>Color{r:0.5764705882352941,g:0.4392156862745098,b:0.8588235294117647,a:1.0},
        "mediumseagreen"=>Color{r:0.23529411764705882,g:0.7019607843137254,b:0.44313725490196076,a:1.0},
        "mediumslateblue"=>Color{r:0.4823529411764706,g:0.40784313725490196,b:0.9333333333333333,a:1.0},
        "mediumspringgreen"=>Color{r:0.0,g:0.9803921568627451,b:0.6039215686274509,a:1.0},
        "mediumturquoise"=>Color{r:0.2823529411764706,g:0.8196078431372549,b:0.8,a:1.0},
        "mediumvioletred"=>Color{r:0.7803921568627451,g:0.08235294117647059,b:0.5215686274509804,a:1.0},
        "midnightblue"=>Color{r:0.09803921568627451,g:0.09803921568627451,b:0.4392156862745098,a:1.0},
        "mintcream"=>Color{r:0.9607843137254902,g:1.0,b:0.9803921568627451,a:1.0},
        "mistyrose"=>Color{r:1.0,g:0.8941176470588236,b:0.8823529411764706,a:1.0},
        "moccasin"=>Color{r:1.0,g:0.8941176470588236,b:0.7098039215686275,a:1.0},
        "navajowhite"=>Color{r:1.0,g:0.8705882352941177,b:0.6784313725490196,a:1.0},
        "navyblue"=>Color{r:0.0,g:0.0,b:0.5019607843137255,a:1.0},
        "oldlace"=>Color{r:0.9921568627450981,g:0.9607843137254902,b:0.9019607843137255,a:1.0},
        "olive"=>Color{r:0.5019607843137255,g:0.5019607843137255,b:0.0,a:1.0},
        "olivedrab"=>Color{r:0.4196078431372549,g:0.5568627450980392,b:0.13725490196078433,a:1.0},
        "orange"=>Color{r:1.0,g:0.6470588235294118,b:0.0,a:1.0},
        "orangered"=>Color{r:1.0,g:0.27058823529411763,b:0.0,a:1.0},
        "orchid"=>Color{r:0.8549019607843137,g:0.4392156862745098,b:0.8392156862745098,a:1.0},
        "palegoldenrod"=>Color{r:0.9333333333333333,g:0.9098039215686274,b:0.6666666666666666,a:1.0},
        "palegreen"=>Color{r:0.596078431372549,g:0.984313725490196,b:0.596078431372549,a:1.0},
        "paleturquoise"=>Color{r:0.6862745098039216,g:0.9333333333333333,b:0.9333333333333333,a:1.0},
        "palevioletred"=>Color{r:0.8588235294117647,g:0.4392156862745098,b:0.5764705882352941,a:1.0},
        "papayawhip"=>Color{r:1.0,g:0.9372549019607843,b:0.8352941176470589,a:1.0},
        "peachpuff"=>Color{r:1.0,g:0.8549019607843137,b:0.7254901960784313,a:1.0},
        "peru"=>Color{r:0.803921568627451,g:0.5215686274509804,b:0.24705882352941178,a:1.0},
        "pink"=>Color{r:1.0,g:0.7529411764705882,b:0.796078431372549,a:1.0},
        "plum"=>Color{r:0.8666666666666667,g:0.6274509803921569,b:0.8666666666666667,a:1.0},
        "powderblue"=>Color{r:0.6901960784313725,g:0.8784313725490196,b:0.9019607843137255,a:1.0},
        "purple"=>Color{r:0.6274509803921569,g:0.12549019607843137,b:0.9411764705882353,a:1.0},
        "webpurple"=>Color{r:0.4980392156862745,g:0.0,b:0.4980392156862745,a:1.0},
        "rebeccapurple"=>Color{r:0.4,g:0.2,b:0.6,a:1.0},
        "red"=>Color{r:1.0,g:0.0,b:0.0,a:1.0},
        "rosybrown"=>Color{r:0.7372549019607844,g:0.5607843137254902,b:0.5607843137254902,a:1.0},
        "royalblue"=>Color{r:0.2549019607843137,g:0.4117647058823529,b:0.8823529411764706,a:1.0},
        "saddlebrown"=>Color{r:0.5450980392156862,g:0.27058823529411763,b:0.07450980392156863,a:1.0},
        "salmon"=>Color{r:0.9803921568627451,g:0.5019607843137255,b:0.4470588235294118,a:1.0},
        "sandybrown"=>Color{r:0.9568627450980393,g:0.6431372549019608,b:0.3764705882352941,a:1.0},
        "seagreen"=>Color{r:0.1803921568627451,g:0.5450980392156862,b:0.3411764705882353,a:1.0},
        "seashell"=>Color{r:1.0,g:0.9607843137254902,b:0.9333333333333333,a:1.0},
        "sienna"=>Color{r:0.6274509803921569,g:0.3215686274509804,b:0.17647058823529413,a:1.0},
        "silver"=>Color{r:0.7529411764705882,g:0.7529411764705882,b:0.7529411764705882,a:1.0},
        "skyblue"=>Color{r:0.5294117647058824,g:0.807843137254902,b:0.9215686274509803,a:1.0},
        "slateblue"=>Color{r:0.41568627450980394,g:0.35294117647058826,b:0.803921568627451,a:1.0},
        "slategray"=>Color{r:0.4392156862745098,g:0.5019607843137255,b:0.5647058823529412,a:1.0},
        "snow"=>Color{r:1.0,g:0.9803921568627451,b:0.9803921568627451,a:1.0},
        "springgreen"=>Color{r:0.0,g:1.0,b:0.4980392156862745,a:1.0},
        "steelblue"=>Color{r:0.27450980392156865,g:0.5098039215686274,b:0.7058823529411765,a:1.0},
        "tan"=>Color{r:0.8235294117647058,g:0.7058823529411765,b:0.5490196078431373,a:1.0},
        "teal"=>Color{r:0.0,g:0.5019607843137255,b:0.5019607843137255,a:1.0},
        "thistle"=>Color{r:0.8470588235294118,g:0.7490196078431373,b:0.8470588235294118,a:1.0},
        "tomato"=>Color{r:1.0,g:0.38823529411764707,b:0.2784313725490196,a:1.0},
        "turquoise"=>Color{r:0.25098039215686274,g:0.8784313725490196,b:0.8156862745098039,a:1.0},
        "violet"=>Color{r:0.9333333333333333,g:0.5098039215686274,b:0.9333333333333333,a:1.0},
        "wheat"=>Color{r:0.9607843137254902,g:0.8705882352941177,b:0.7019607843137254,a:1.0},
        "white"=>Color{r:1.0,g:1.0,b:1.0,a:1.0},
        "whitesmoke"=>Color{r:0.9607843137254902,g:0.9607843137254902,b:0.9607843137254902,a:1.0},
        "yellow"=>Color{r:1.0,g:1.0,b:0.0,a:1.0},
        "yellowgreen"=>Color{r:0.6039215686274509,g:0.803921568627451,b:0.19607843137254902,a:1.0},
        "red500"=>Color{r:0.9568627450980393,g:0.2627450980392157,b:0.21176470588235294,a:1.0},
        "red50"=>Color{r:1.0,g:0.9215686274509803,b:0.9333333333333333,a:1.0},
        "red100"=>Color{r:1.0,g:0.803921568627451,b:0.8235294117647058,a:1.0},
        "red200"=>Color{r:0.9372549019607843,g:0.6039215686274509,b:0.6039215686274509,a:1.0},
        "red300"=>Color{r:0.8980392156862745,g:0.45098039215686275,b:0.45098039215686275,a:1.0},
        "red400"=>Color{r:0.9372549019607843,g:0.3254901960784314,b:0.3137254901960784,a:1.0},
        "red600"=>Color{r:0.8980392156862745,g:0.2235294117647059,b:0.20784313725490197,a:1.0},
        "red700"=>Color{r:0.8274509803921568,g:0.1843137254901961,b:0.1843137254901961,a:1.0},
        "red800"=>Color{r:0.7764705882352941,g:0.1568627450980392,b:0.1568627450980392,a:1.0},
        "red900"=>Color{r:0.7176470588235294,g:0.10980392156862745,b:0.10980392156862745,a:1.0},
        "reda100"=>Color{r:1.0,g:0.5411764705882353,b:0.5019607843137255,a:1.0},
        "reda200"=>Color{r:1.0,g:0.3215686274509804,b:0.3215686274509804,a:1.0},
        "reda400"=>Color{r:1.0,g:0.09019607843137255,b:0.26666666666666666,a:1.0},
        "reda700"=>Color{r:0.8352941176470589,g:0.0,b:0.0,a:1.0},
        "pink500"=>Color{r:0.9137254901960784,g:0.11764705882352941,b:0.38823529411764707,a:1.0},
        "pink50"=>Color{r:0.9882352941176471,g:0.8941176470588236,b:0.9254901960784314,a:1.0},
        "pink100"=>Color{r:0.9725490196078431,g:0.7333333333333333,b:0.8156862745098039,a:1.0},
        "pink200"=>Color{r:0.9568627450980393,g:0.5607843137254902,b:0.6941176470588235,a:1.0},
        "pink300"=>Color{r:0.9411764705882353,g:0.3843137254901961,b:0.5725490196078431,a:1.0},
        "pink400"=>Color{r:0.9254901960784314,g:0.25098039215686274,b:0.47843137254901963,a:1.0},
        "pink600"=>Color{r:0.8470588235294118,g:0.10588235294117647,b:0.3764705882352941,a:1.0},
        "pink700"=>Color{r:0.7607843137254902,g:0.09411764705882353,b:0.3568627450980392,a:1.0},
        "pink800"=>Color{r:0.6784313725490196,g:0.0784313725490196,b:0.3411764705882353,a:1.0},
        "pink900"=>Color{r:0.5333333333333333,g:0.054901960784313725,b:0.30980392156862746,a:1.0},
        "pinka100"=>Color{r:1.0,g:0.5019607843137255,b:0.6705882352941176,a:1.0},
        "pinka200"=>Color{r:1.0,g:0.25098039215686274,b:0.5058823529411764,a:1.0},
        "pinka400"=>Color{r:0.9607843137254902,g:0.0,b:0.3411764705882353,a:1.0},
        "pinka700"=>Color{r:0.7725490196078432,g:0.06666666666666667,b:0.3843137254901961,a:1.0},
        "purple500"=>Color{r:0.611764705882353,g:0.15294117647058825,b:0.6901960784313725,a:1.0},
        "purple50"=>Color{r:0.9529411764705882,g:0.8980392156862745,b:0.9607843137254902,a:1.0},
        "purple100"=>Color{r:0.8823529411764706,g:0.7450980392156863,b:0.9058823529411765,a:1.0},
        "purple200"=>Color{r:0.807843137254902,g:0.5764705882352941,b:0.8470588235294118,a:1.0},
        "purple300"=>Color{r:0.7294117647058823,g:0.40784313725490196,b:0.7843137254901961,a:1.0},
        "purple400"=>Color{r:0.6705882352941176,g:0.2784313725490196,b:0.7372549019607844,a:1.0},
        "purple600"=>Color{r:0.5568627450980392,g:0.1411764705882353,b:0.6666666666666666,a:1.0},
        "purple700"=>Color{r:0.4823529411764706,g:0.12156862745098039,b:0.6352941176470588,a:1.0},
        "purple800"=>Color{r:0.41568627450980394,g:0.10588235294117647,b:0.6039215686274509,a:1.0},
        "purple900"=>Color{r:0.2901960784313726,g:0.0784313725490196,b:0.5490196078431373,a:1.0},
        "purplea100"=>Color{r:0.9176470588235294,g:0.5019607843137255,b:0.9882352941176471,a:1.0},
        "purplea200"=>Color{r:0.8784313725490196,g:0.25098039215686274,b:0.984313725490196,a:1.0},
        "purplea400"=>Color{r:0.8352941176470589,g:0.0,b:0.9764705882352941,a:1.0},
        "purplea700"=>Color{r:0.6666666666666666,g:0.0,b:1.0,a:1.0},
        "deeppurple500"=>Color{r:0.403921568627451,g:0.22745098039215686,b:0.7176470588235294,a:1.0},
        "deeppurple50"=>Color{r:0.9294117647058824,g:0.9058823529411765,b:0.9647058823529412,a:1.0},
        "deeppurple100"=>Color{r:0.8196078431372549,g:0.7686274509803922,b:0.9137254901960784,a:1.0},
        "deeppurple200"=>Color{r:0.7019607843137254,g:0.615686274509804,b:0.8588235294117647,a:1.0},
        "deeppurple300"=>Color{r:0.5843137254901961,g:0.4588235294117647,b:0.803921568627451,a:1.0},
        "deeppurple400"=>Color{r:0.49411764705882355,g:0.3411764705882353,b:0.7607843137254902,a:1.0},
        "deeppurple600"=>Color{r:0.3686274509803922,g:0.20784313725490197,b:0.6941176470588235,a:1.0},
        "deeppurple700"=>Color{r:0.3176470588235294,g:0.17647058823529413,b:0.6588235294117647,a:1.0},
        "deeppurple800"=>Color{r:0.27058823529411763,g:0.15294117647058825,b:0.6274509803921569,a:1.0},
        "deeppurple900"=>Color{r:0.19215686274509805,g:0.10588235294117647,b:0.5725490196078431,a:1.0},
        "deeppurplea100"=>Color{r:0.7019607843137254,g:0.5333333333333333,b:1.0,a:1.0},
        "deeppurplea200"=>Color{r:0.48627450980392156,g:0.30196078431372547,b:1.0,a:1.0},
        "deeppurplea400"=>Color{r:0.396078431372549,g:0.12156862745098039,b:1.0,a:1.0},
        "deeppurplea700"=>Color{r:0.3843137254901961,g:0.0,b:0.9176470588235294,a:1.0},
        "indigo500"=>Color{r:0.24705882352941178,g:0.3176470588235294,b:0.7098039215686275,a:1.0},
        "indigo50"=>Color{r:0.9098039215686274,g:0.9176470588235294,b:0.9647058823529412,a:1.0},
        "indigo100"=>Color{r:0.7725490196078432,g:0.792156862745098,b:0.9137254901960784,a:1.0},
        "indigo200"=>Color{r:0.6235294117647059,g:0.6588235294117647,b:0.8549019607843137,a:1.0},
        "indigo300"=>Color{r:0.4745098039215686,g:0.5254901960784314,b:0.796078431372549,a:1.0},
        "indigo400"=>Color{r:0.3607843137254902,g:0.4196078431372549,b:0.7529411764705882,a:1.0},
        "indigo600"=>Color{r:0.2235294117647059,g:0.28627450980392155,b:0.6705882352941176,a:1.0},
        "indigo700"=>Color{r:0.18823529411764706,g:0.24705882352941178,b:0.6235294117647059,a:1.0},
        "indigo800"=>Color{r:0.1568627450980392,g:0.20784313725490197,b:0.5764705882352941,a:1.0},
        "indigo900"=>Color{r:0.10196078431372549,g:0.13725490196078433,b:0.49411764705882355,a:1.0},
        "indigoa100"=>Color{r:0.5490196078431373,g:0.6196078431372549,b:1.0,a:1.0},
        "indigoa200"=>Color{r:0.3254901960784314,g:0.42745098039215684,b:0.996078431372549,a:1.0},
        "indigoa400"=>Color{r:0.23921568627450981,g:0.35294117647058826,b:0.996078431372549,a:1.0},
        "indigoa700"=>Color{r:0.18823529411764706,g:0.30980392156862746,b:0.996078431372549,a:1.0},
        "blue500"=>Color{r:0.12941176470588237,g:0.5882352941176471,b:0.9529411764705882,a:1.0},
        "blue50"=>Color{r:0.8901960784313725,g:0.9490196078431372,b:0.9921568627450981,a:1.0},
        "blue100"=>Color{r:0.7333333333333333,g:0.8705882352941177,b:0.984313725490196,a:1.0},
        "blue200"=>Color{r:0.5647058823529412,g:0.792156862745098,b:0.9764705882352941,a:1.0},
        "blue300"=>Color{r:0.39215686274509803,g:0.7098039215686275,b:0.9647058823529412,a:1.0},
        "blue400"=>Color{r:0.25882352941176473,g:0.6470588235294118,b:0.9607843137254902,a:1.0},
        "blue600"=>Color{r:0.11764705882352941,g:0.5333333333333333,b:0.8980392156862745,a:1.0},
        "blue700"=>Color{r:0.09803921568627451,g:0.4627450980392157,b:0.8235294117647058,a:1.0},
        "blue800"=>Color{r:0.08235294117647059,g:0.396078431372549,b:0.7529411764705882,a:1.0},
        "blue900"=>Color{r:0.050980392156862744,g:0.2784313725490196,b:0.6313725490196078,a:1.0},
        "bluea100"=>Color{r:0.5098039215686274,g:0.6941176470588235,b:1.0,a:1.0},
        "bluea200"=>Color{r:0.26666666666666666,g:0.5411764705882353,b:1.0,a:1.0},
        "bluea400"=>Color{r:0.1607843137254902,g:0.4745098039215686,b:1.0,a:1.0},
        "bluea700"=>Color{r:0.1607843137254902,g:0.3843137254901961,b:1.0,a:1.0},
        "lightblue500"=>Color{r:0.011764705882352941,g:0.6627450980392157,b:0.9568627450980393,a:1.0},
        "lightblue50"=>Color{r:0.8823529411764706,g:0.9607843137254902,b:0.996078431372549,a:1.0},
        "lightblue100"=>Color{r:0.7019607843137254,g:0.8980392156862745,b:0.9882352941176471,a:1.0},
        "lightblue200"=>Color{r:0.5058823529411764,g:0.8313725490196079,b:0.9803921568627451,a:1.0},
        "lightblue300"=>Color{r:0.30980392156862746,g:0.7647058823529411,b:0.9686274509803922,a:1.0},
        "lightblue400"=>Color{r:0.1607843137254902,g:0.7137254901960784,b:0.9647058823529412,a:1.0},
        "lightblue600"=>Color{r:0.011764705882352941,g:0.6078431372549019,b:0.8980392156862745,a:1.0},
        "lightblue700"=>Color{r:0.00784313725490196,g:0.5333333333333333,b:0.8196078431372549,a:1.0},
        "lightblue800"=>Color{r:0.00784313725490196,g:0.4666666666666667,b:0.7411764705882353,a:1.0},
        "lightblue900"=>Color{r:0.00392156862745098,g:0.3411764705882353,b:0.6078431372549019,a:1.0},
        "lightbluea100"=>Color{r:0.5019607843137255,g:0.8470588235294118,b:1.0,a:1.0},
        "lightbluea200"=>Color{r:0.25098039215686274,g:0.7686274509803922,b:1.0,a:1.0},
        "lightbluea400"=>Color{r:0.0,g:0.6901960784313725,b:1.0,a:1.0},
        "lightbluea700"=>Color{r:0.0,g:0.5686274509803921,b:0.9176470588235294,a:1.0},
        "cyan500"=>Color{r:0.0,g:0.7372549019607844,b:0.8313725490196079,a:1.0},
        "cyan50"=>Color{r:0.8784313725490196,g:0.9686274509803922,b:0.9803921568627451,a:1.0},
        "cyan100"=>Color{r:0.6980392156862745,g:0.9215686274509803,b:0.9490196078431372,a:1.0},
        "cyan200"=>Color{r:0.5019607843137255,g:0.8705882352941177,b:0.9176470588235294,a:1.0},
        "cyan300"=>Color{r:0.30196078431372547,g:0.8156862745098039,b:0.8823529411764706,a:1.0},
        "cyan400"=>Color{r:0.14901960784313725,g:0.7764705882352941,b:0.8549019607843137,a:1.0},
        "cyan600"=>Color{r:0.0,g:0.6745098039215687,b:0.7568627450980392,a:1.0},
        "cyan700"=>Color{r:0.0,g:0.592156862745098,b:0.6549019607843137,a:1.0},
        "cyan800"=>Color{r:0.0,g:0.5137254901960784,b:0.5607843137254902,a:1.0},
        "cyan900"=>Color{r:0.0,g:0.3764705882352941,b:0.39215686274509803,a:1.0},
        "cyana100"=>Color{r:0.5176470588235295,g:1.0,b:1.0,a:1.0},
        "cyana200"=>Color{r:0.09411764705882353,g:1.0,b:1.0,a:1.0},
        "cyana400"=>Color{r:0.0,g:0.8980392156862745,b:1.0,a:1.0},
        "cyana700"=>Color{r:0.0,g:0.7215686274509804,b:0.8313725490196079,a:1.0},
        "teal500"=>Color{r:0.0,g:0.5882352941176471,b:0.5333333333333333,a:1.0},
        "teal50"=>Color{r:0.8784313725490196,g:0.9490196078431372,b:0.9450980392156862,a:1.0},
        "teal100"=>Color{r:0.6980392156862745,g:0.8745098039215686,b:0.8588235294117647,a:1.0},
        "teal200"=>Color{r:0.5019607843137255,g:0.796078431372549,b:0.7686274509803922,a:1.0},
        "teal300"=>Color{r:0.30196078431372547,g:0.7137254901960784,b:0.6745098039215687,a:1.0},
        "teal400"=>Color{r:0.14901960784313725,g:0.6509803921568628,b:0.6039215686274509,a:1.0},
        "teal600"=>Color{r:0.0,g:0.5372549019607843,b:0.4823529411764706,a:1.0},
        "teal700"=>Color{r:0.0,g:0.4745098039215686,b:0.4196078431372549,a:1.0},
        "teal800"=>Color{r:0.0,g:0.4117647058823529,b:0.3607843137254902,a:1.0},
        "teal900"=>Color{r:0.0,g:0.30196078431372547,b:0.25098039215686274,a:1.0},
        "teala100"=>Color{r:0.6549019607843137,g:1.0,b:0.9215686274509803,a:1.0},
        "teala200"=>Color{r:0.39215686274509803,g:1.0,b:0.8549019607843137,a:1.0},
        "teala400"=>Color{r:0.11372549019607843,g:0.9137254901960784,b:0.7137254901960784,a:1.0},
        "teala700"=>Color{r:0.0,g:0.7490196078431373,b:0.6470588235294118,a:1.0},
        "green500"=>Color{r:0.2980392156862745,g:0.6862745098039216,b:0.3137254901960784,a:1.0},
        "green50"=>Color{r:0.9098039215686274,g:0.9607843137254902,b:0.9137254901960784,a:1.0},
        "green100"=>Color{r:0.7843137254901961,g:0.9019607843137255,b:0.788235294117647,a:1.0},
        "green200"=>Color{r:0.6470588235294118,g:0.8392156862745098,b:0.6549019607843137,a:1.0},
        "green300"=>Color{r:0.5058823529411764,g:0.7803921568627451,b:0.5176470588235295,a:1.0},
        "green400"=>Color{r:0.4,g:0.7333333333333333,b:0.41568627450980394,a:1.0},
        "green600"=>Color{r:0.2627450980392157,g:0.6274509803921569,b:0.2784313725490196,a:1.0},
        "green700"=>Color{r:0.2196078431372549,g:0.5568627450980392,b:0.23529411764705882,a:1.0},
        "green800"=>Color{r:0.1803921568627451,g:0.49019607843137253,b:0.19607843137254902,a:1.0},
        "green900"=>Color{r:0.10588235294117647,g:0.3686274509803922,b:0.12549019607843137,a:1.0},
        "greena100"=>Color{r:0.7254901960784313,g:0.9647058823529412,b:0.792156862745098,a:1.0},
        "greena200"=>Color{r:0.4117647058823529,g:0.9411764705882353,b:0.6823529411764706,a:1.0},
        "greena400"=>Color{r:0.0,g:0.9019607843137255,b:0.4627450980392157,a:1.0},
        "greena700"=>Color{r:0.0,g:0.7843137254901961,b:0.3254901960784314,a:1.0},
        "lightgreen500"=>Color{r:0.5450980392156862,g:0.7647058823529411,b:0.2901960784313726,a:1.0},
        "lightgreen50"=>Color{r:0.9450980392156862,g:0.9725490196078431,b:0.9137254901960784,a:1.0},
        "lightgreen100"=>Color{r:0.8627450980392157,g:0.9294117647058824,b:0.7843137254901961,a:1.0},
        "lightgreen200"=>Color{r:0.7725490196078432,g:0.8823529411764706,b:0.6470588235294118,a:1.0},
        "lightgreen300"=>Color{r:0.6823529411764706,g:0.8352941176470589,b:0.5058823529411764,a:1.0},
        "lightgreen400"=>Color{r:0.611764705882353,g:0.8,b:0.396078431372549,a:1.0},
        "lightgreen600"=>Color{r:0.48627450980392156,g:0.7019607843137254,b:0.25882352941176473,a:1.0},
        "lightgreen700"=>Color{r:0.40784313725490196,g:0.6235294117647059,b:0.2196078431372549,a:1.0},
        "lightgreen800"=>Color{r:0.3333333333333333,g:0.5450980392156862,b:0.1843137254901961,a:1.0},
        "lightgreen900"=>Color{r:0.2,g:0.4117647058823529,b:0.11764705882352941,a:1.0},
        "lightgreena100"=>Color{r:0.8,g:1.0,b:0.5647058823529412,a:1.0},
        "lightgreena200"=>Color{r:0.6980392156862745,g:1.0,b:0.34901960784313724,a:1.0},
        "lightgreena400"=>Color{r:0.4627450980392157,g:1.0,b:0.011764705882352941,a:1.0},
        "lightgreena700"=>Color{r:0.39215686274509803,g:0.8666666666666667,b:0.09019607843137255,a:1.0},
        "lime500"=>Color{r:0.803921568627451,g:0.8627450980392157,b:0.2235294117647059,a:1.0},
        "lime50"=>Color{r:0.9764705882352941,g:0.984313725490196,b:0.9058823529411765,a:1.0},
        "lime100"=>Color{r:0.9411764705882353,g:0.9568627450980393,b:0.7647058823529411,a:1.0},
        "lime200"=>Color{r:0.9019607843137255,g:0.9333333333333333,b:0.611764705882353,a:1.0},
        "lime300"=>Color{r:0.8627450980392157,g:0.9058823529411765,b:0.4588235294117647,a:1.0},
        "lime400"=>Color{r:0.8313725490196079,g:0.8823529411764706,b:0.3411764705882353,a:1.0},
        "lime600"=>Color{r:0.7529411764705882,g:0.792156862745098,b:0.2,a:1.0},
        "lime700"=>Color{r:0.6862745098039216,g:0.7058823529411765,b:0.16862745098039217,a:1.0},
        "lime800"=>Color{r:0.6196078431372549,g:0.615686274509804,b:0.1411764705882353,a:1.0},
        "lime900"=>Color{r:0.5098039215686274,g:0.4666666666666667,b:0.09019607843137255,a:1.0},
        "limea100"=>Color{r:0.9568627450980393,g:1.0,b:0.5058823529411764,a:1.0},
        "limea200"=>Color{r:0.9333333333333333,g:1.0,b:0.2549019607843137,a:1.0},
        "limea400"=>Color{r:0.7764705882352941,g:1.0,b:0.0,a:1.0},
        "limea700"=>Color{r:0.6823529411764706,g:0.9176470588235294,b:0.0,a:1.0},
        "yellow500"=>Color{r:1.0,g:0.9215686274509803,b:0.23137254901960785,a:1.0},
        "yellow50"=>Color{r:1.0,g:0.9921568627450981,b:0.9058823529411765,a:1.0},
        "yellow100"=>Color{r:1.0,g:0.9764705882352941,b:0.7686274509803922,a:1.0},
        "yellow200"=>Color{r:1.0,g:0.9607843137254902,b:0.615686274509804,a:1.0},
        "yellow300"=>Color{r:1.0,g:0.9450980392156862,b:0.4627450980392157,a:1.0},
        "yellow400"=>Color{r:1.0,g:0.9333333333333333,b:0.34509803921568627,a:1.0},
        "yellow600"=>Color{r:0.9921568627450981,g:0.8470588235294118,b:0.20784313725490197,a:1.0},
        "yellow700"=>Color{r:0.984313725490196,g:0.7529411764705882,b:0.17647058823529413,a:1.0},
        "yellow800"=>Color{r:0.9764705882352941,g:0.6588235294117647,b:0.1450980392156863,a:1.0},
        "yellow900"=>Color{r:0.9607843137254902,g:0.4980392156862745,b:0.09019607843137255,a:1.0},
        "yellowa100"=>Color{r:1.0,g:1.0,b:0.5529411764705883,a:1.0},
        "yellowa200"=>Color{r:1.0,g:1.0,b:0.0,a:1.0},
        "yellowa400"=>Color{r:1.0,g:0.9176470588235294,b:0.0,a:1.0},
        "yellowa700"=>Color{r:1.0,g:0.8392156862745098,b:0.0,a:1.0},
        "amber500"=>Color{r:1.0,g:0.7568627450980392,b:0.027450980392156862,a:1.0},
        "amber50"=>Color{r:1.0,g:0.9725490196078431,b:0.8823529411764706,a:1.0},
        "amber100"=>Color{r:1.0,g:0.9254901960784314,b:0.7019607843137254,a:1.0},
        "amber200"=>Color{r:1.0,g:0.8784313725490196,b:0.5098039215686274,a:1.0},
        "amber300"=>Color{r:1.0,g:0.8352941176470589,b:0.30980392156862746,a:1.0},
        "amber400"=>Color{r:1.0,g:0.792156862745098,b:0.1568627450980392,a:1.0},
        "amber600"=>Color{r:1.0,g:0.7019607843137254,b:0.0,a:1.0},
        "amber700"=>Color{r:1.0,g:0.6274509803921569,b:0.0,a:1.0},
        "amber800"=>Color{r:1.0,g:0.5607843137254902,b:0.0,a:1.0},
        "amber900"=>Color{r:1.0,g:0.43529411764705883,b:0.0,a:1.0},
        "ambera100"=>Color{r:1.0,g:0.8980392156862745,b:0.4980392156862745,a:1.0},
        "ambera200"=>Color{r:1.0,g:0.8431372549019608,b:0.25098039215686274,a:1.0},
        "ambera400"=>Color{r:1.0,g:0.7686274509803922,b:0.0,a:1.0},
        "ambera700"=>Color{r:1.0,g:0.6705882352941176,b:0.0,a:1.0},
        "orange500"=>Color{r:1.0,g:0.596078431372549,b:0.0,a:1.0},
        "orange50"=>Color{r:1.0,g:0.9529411764705882,b:0.8784313725490196,a:1.0},
        "orange100"=>Color{r:1.0,g:0.8784313725490196,b:0.6980392156862745,a:1.0},
        "orange200"=>Color{r:1.0,g:0.8,b:0.5019607843137255,a:1.0},
        "orange300"=>Color{r:1.0,g:0.7176470588235294,b:0.30196078431372547,a:1.0},
        "orange400"=>Color{r:1.0,g:0.6549019607843137,b:0.14901960784313725,a:1.0},
        "orange600"=>Color{r:0.984313725490196,g:0.5490196078431373,b:0.0,a:1.0},
        "orange700"=>Color{r:0.9607843137254902,g:0.48627450980392156,b:0.0,a:1.0},
        "orange800"=>Color{r:0.9372549019607843,g:0.4235294117647059,b:0.0,a:1.0},
        "orange900"=>Color{r:0.9019607843137255,g:0.3176470588235294,b:0.0,a:1.0},
        "orangea100"=>Color{r:1.0,g:0.8196078431372549,b:0.5019607843137255,a:1.0},
        "orangea200"=>Color{r:1.0,g:0.6705882352941176,b:0.25098039215686274,a:1.0},
        "orangea400"=>Color{r:1.0,g:0.5686274509803921,b:0.0,a:1.0},
        "orangea700"=>Color{r:1.0,g:0.42745098039215684,b:0.0,a:1.0},
        "deeporange500"=>Color{r:1.0,g:0.3411764705882353,b:0.13333333333333333,a:1.0},
        "deeporange50"=>Color{r:0.984313725490196,g:0.9137254901960784,b:0.9058823529411765,a:1.0},
        "deeporange100"=>Color{r:1.0,g:0.8,b:0.7372549019607844,a:1.0},
        "deeporange200"=>Color{r:1.0,g:0.6705882352941176,b:0.5686274509803921,a:1.0},
        "deeporange300"=>Color{r:1.0,g:0.5411764705882353,b:0.396078431372549,a:1.0},
        "deeporange400"=>Color{r:1.0,g:0.4392156862745098,b:0.2627450980392157,a:1.0},
        "deeporange600"=>Color{r:0.9568627450980393,g:0.3176470588235294,b:0.11764705882352941,a:1.0},
        "deeporange700"=>Color{r:0.9019607843137255,g:0.2901960784313726,b:0.09803921568627451,a:1.0},
        "deeporange800"=>Color{r:0.8470588235294118,g:0.2627450980392157,b:0.08235294117647059,a:1.0},
        "deeporange900"=>Color{r:0.7490196078431373,g:0.21176470588235294,b:0.047058823529411764,a:1.0},
        "deeporangea100"=>Color{r:1.0,g:0.6196078431372549,b:0.5019607843137255,a:1.0},
        "deeporangea200"=>Color{r:1.0,g:0.43137254901960786,b:0.25098039215686274,a:1.0},
        "deeporangea400"=>Color{r:1.0,g:0.23921568627450981,b:0.0,a:1.0},
        "deeporangea700"=>Color{r:0.8666666666666667,g:0.17254901960784313,b:0.0,a:1.0},
        "brown500"=>Color{r:0.4745098039215686,g:0.3333333333333333,b:0.2823529411764706,a:1.0},
        "brown50"=>Color{r:0.9372549019607843,g:0.9215686274509803,b:0.9137254901960784,a:1.0},
        "brown100"=>Color{r:0.8431372549019608,g:0.8,b:0.7843137254901961,a:1.0},
        "brown200"=>Color{r:0.7372549019607844,g:0.6666666666666666,b:0.6431372549019608,a:1.0},
        "brown300"=>Color{r:0.6313725490196078,g:0.5333333333333333,b:0.4980392156862745,a:1.0},
        "brown400"=>Color{r:0.5529411764705883,g:0.43137254901960786,b:0.38823529411764707,a:1.0},
        "brown600"=>Color{r:0.42745098039215684,g:0.2980392156862745,b:0.2549019607843137,a:1.0},
        "brown700"=>Color{r:0.36470588235294116,g:0.25098039215686274,b:0.21568627450980393,a:1.0},
        "brown800"=>Color{r:0.3058823529411765,g:0.20392156862745098,b:0.1803921568627451,a:1.0},
        "brown900"=>Color{r:0.24313725490196078,g:0.15294117647058825,b:0.13725490196078433,a:1.0},
        "grey500"=>Color{r:0.6196078431372549,g:0.6196078431372549,b:0.6196078431372549,a:1.0},
        "grey50"=>Color{r:0.9803921568627451,g:0.9803921568627451,b:0.9803921568627451,a:1.0},
        "grey100"=>Color{r:0.9607843137254902,g:0.9607843137254902,b:0.9607843137254902,a:1.0},
        "grey200"=>Color{r:0.9333333333333333,g:0.9333333333333333,b:0.9333333333333333,a:1.0},
        "grey300"=>Color{r:0.8784313725490196,g:0.8784313725490196,b:0.8784313725490196,a:1.0},
        "grey400"=>Color{r:0.7411764705882353,g:0.7411764705882353,b:0.7411764705882353,a:1.0},
        "grey600"=>Color{r:0.4588235294117647,g:0.4588235294117647,b:0.4588235294117647,a:1.0},
        "grey700"=>Color{r:0.3803921568627451,g:0.3803921568627451,b:0.3803921568627451,a:1.0},
        "grey800"=>Color{r:0.25882352941176473,g:0.25882352941176473,b:0.25882352941176473,a:1.0},
        "grey850"=>Color{r:0.19215686274509805,g:0.19215686274509805,b:0.19215686274509805,a:1.0},
        "grey900"=>Color{r:0.12941176470588237,g:0.12941176470588237,b:0.12941176470588237,a:1.0},
        "bluegrey500"=>Color{r:0.3764705882352941,g:0.49019607843137253,b:0.5450980392156862,a:1.0},
        "bluegrey50"=>Color{r:0.9254901960784314,g:0.9372549019607843,b:0.9450980392156862,a:1.0},
        "bluegrey100"=>Color{r:0.8117647058823529,g:0.8470588235294118,b:0.8627450980392157,a:1.0},
        "bluegrey200"=>Color{r:0.6901960784313725,g:0.7450980392156863,b:0.7725490196078432,a:1.0},
        "bluegrey300"=>Color{r:0.5647058823529412,g:0.6431372549019608,b:0.6823529411764706,a:1.0},
        "bluegrey400"=>Color{r:0.47058823529411764,g:0.5647058823529412,b:0.611764705882353,a:1.0},
        "bluegrey600"=>Color{r:0.32941176470588235,g:0.43137254901960786,b:0.47843137254901963,a:1.0},
        "bluegrey700"=>Color{r:0.27058823529411763,g:0.35294117647058826,b:0.39215686274509803,a:1.0},
        "bluegrey800"=>Color{r:0.21568627450980393,g:0.2784313725490196,b:0.30980392156862746,a:1.0},
        "bluegrey900"=>Color{r:0.14901960784313725,g:0.19607843137254902,b:0.2196078431372549,a:1.0},
        color=>{
            let bytes = color.as_bytes();
            if bytes[0] == '#' as u8{
                match bytes.len(){
                    2=>{ // #c
                        let val = hex_to_int(bytes[1] as u32) as f32 / 15.0;
                        return Color{r:val, g:val, b:val, a:1.0}
                    },
                    4=>{ // #abc
                        let r = hex_to_int(bytes[1] as u32) as f32 / 15.0;
                        let g = hex_to_int(bytes[2] as u32) as f32 / 15.0;
                        let b = hex_to_int(bytes[3] as u32) as f32 / 15.0;
                        return  Color{r:r, g:g, b:b, a:1.0}
                    },
                    5=>{ // #abcd
                        let r = hex_to_int(bytes[1] as u32) as f32 / 15.0;
                        let g = hex_to_int(bytes[2] as u32) as f32 / 15.0;
                        let b = hex_to_int(bytes[3] as u32) as f32 / 15.0;
                        let a = hex_to_int(bytes[4] as u32) as f32 / 15.0;
                        return  Color{r:r, g:g, b:b, a:a}
                    },
                    7=>{ // #aabbcc
                        let r = ((hex_to_int(bytes[1] as u32)<<4)+hex_to_int(bytes[2] as u32)) as f32 / 255.0;
                        let g = ((hex_to_int(bytes[3] as u32)<<4)+hex_to_int(bytes[4] as u32)) as f32 / 255.0;
                        let b = ((hex_to_int(bytes[5] as u32)<<4)+hex_to_int(bytes[6] as u32)) as f32 / 255.0;
                        return  Color{r:r, g:g, b:b, a:1.0}
                    },
                    9=>{
                        let r = ((hex_to_int(bytes[1] as u32)<<4)+hex_to_int(bytes[2] as u32)) as f32 / 255.0;
                        let g = ((hex_to_int(bytes[3] as u32)<<4)+hex_to_int(bytes[4] as u32)) as f32 / 255.0;
                        let b = ((hex_to_int(bytes[5] as u32)<<4)+hex_to_int(bytes[6] as u32)) as f32 / 255.0;
                        let a = ((hex_to_int(bytes[7] as u32)<<4)+hex_to_int(bytes[8] as u32)) as f32 / 255.0;
                        return  Color{r:r, g:g, b:b, a:a}
                    }
                    _=>()
                }
            };
            return Color{r:1.0,g:1.0,b:1.0,a:1.0}
        }
    }
}

fn hex_to_int(c:u32)->u32{
    if c>=48 && c <=57{
        return c - 48
    }
    if c>=65 && c <=70{
        return c - 65 + 10
    }
    if c>=97 && c <=102{
        return c - 97 + 10
    }
    return 0
}