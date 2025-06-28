use super::aastructs::*;
use super::font16::aa_font16;

unsafe extern "C" {
    fn printf(_: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    fn memset(
        _: *mut std::ffi::c_void,
        _: std::ffi::c_int,
        _: std::ffi::c_ulong,
    ) -> *mut std::ffi::c_void;
    fn calloc(_: std::ffi::c_ulong, _: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    fn free(_: *mut std::ffi::c_void);
    fn exit(_: std::ffi::c_int) -> !;
    fn abs(_: std::ffi::c_int) -> std::ffi::c_int;
}

pub static mut aa_defparams: aa_hardware_params = {
    let mut init = aa_hardware_params {
        font: 0 as *const aa_font,
        supported: 1 as std::ffi::c_int | 2 as std::ffi::c_int | 4 as std::ffi::c_int,
        minwidth: 0,
        minheight: 0,
        maxwidth: 0,
        maxheight: 0,
        recwidth: 0,
        recheight: 0,
        mmwidth: 0,
        mmheight: 0,
        width: 0,
        height: 0,
        dimmul: 0.,
        boldmul: 0.,
    };
    init
};

pub unsafe extern "C" fn aa_uninitmouse(mut c: *mut aa_context) {
    if !((*c).mousedriver).is_null() {
        ((*(*c).mousedriver).uninit).expect("non-null function pointer")(c);
        if !((*c).mousedriverdata).is_null() {
            free((*c).mousedriverdata);
        }
        (*c).mousedriverdata = 0 as *mut std::ffi::c_void;
        (*c).mousedriver = 0 as *const aa_mousedriver;
        (*c).mousemode = 0 as std::ffi::c_int;
    }
}

pub unsafe extern "C" fn aa_uninitkbd(mut c: *mut aa_context) {
    if !((*c).kbddriver).is_null() {
        if !((*c).mousedriver).is_null() {
            aa_uninitmouse(c);
        }
        (*c).mousedriverdata = 0 as *mut std::ffi::c_void;
        ((*(*c).kbddriver).uninit).expect("non-null function pointer")(c);
        if !((*c).kbddriverdata).is_null() {
            free((*c).kbddriverdata);
        }
        (*c).kbddriverdata = 0 as *mut std::ffi::c_void;
        (*c).kbddriver = 0 as *const aa_kbddriver;
    }
}

pub unsafe extern "C" fn aa_resize(mut c: *mut aa_context) -> std::ffi::c_int {
    let mut width: std::ffi::c_int = 0;
    let mut height: std::ffi::c_int = 0;
    width = abs((*c).params.width);
    height = abs((*c).params.height);
    ((*(*c).driver).getsize).expect("non-null function pointer")(c, &mut width, &mut height);
    if width <= 0 as std::ffi::c_int || height <= 0 as std::ffi::c_int {
        printf(b"Invalid buffer sizes!\n\0" as *const u8 as *const std::ffi::c_char);
        exit(-(1 as std::ffi::c_int));
    }
    if width != (*c).params.width || height != (*c).imgheight {
        if !((*c).imagebuffer).is_null() {
            free((*c).imagebuffer as *mut std::ffi::c_void);
        }
        if !((*c).textbuffer).is_null() {
            free((*c).textbuffer as *mut std::ffi::c_void);
        }
        if !((*c).attrbuffer).is_null() {
            free((*c).attrbuffer as *mut std::ffi::c_void);
        }
        (*c).params.width = width;
        (*c).params.height = height;
        (*c).imgwidth = width * (*c).mulx;
        (*c).imgheight = height * (*c).mulx;
        (*c).imagebuffer = calloc(
            1 as std::ffi::c_int as std::ffi::c_ulong,
            ((*c).imgwidth * (*c).imgheight) as std::ffi::c_ulong,
        ) as *mut std::ffi::c_uchar;
        if ((*c).imagebuffer).is_null() {
            return 0 as std::ffi::c_int;
        }
        (*c).textbuffer = calloc(
            1 as std::ffi::c_int as std::ffi::c_ulong,
            ((*c).params.width * (*c).params.height) as std::ffi::c_ulong,
        ) as *mut std::ffi::c_uchar;
        if ((*c).textbuffer).is_null() {
            free((*c).imagebuffer as *mut std::ffi::c_void);
            return 0 as std::ffi::c_int;
        }
        memset(
            (*c).textbuffer as *mut std::ffi::c_void,
            ' ' as i32,
            ((*c).params.width * (*c).params.height) as std::ffi::c_ulong,
        );
        (*c).attrbuffer = calloc(
            1 as std::ffi::c_int as std::ffi::c_ulong,
            ((*c).params.width * (*c).params.height) as std::ffi::c_ulong,
        ) as *mut std::ffi::c_uchar;
        if ((*c).attrbuffer).is_null() {
            free((*c).imagebuffer as *mut std::ffi::c_void);
            free((*c).textbuffer as *mut std::ffi::c_void);
            return 0 as std::ffi::c_int;
        }
    }
    if (*c).driverparams.mmwidth == 0 {
        (*c).params.mmwidth = 290 as std::ffi::c_int;
    } else {
        (*c).params.mmwidth = (*c).driverparams.mmwidth;
    }
    if (*c).driverparams.mmheight == 0 {
        (*c).params.mmheight = 215 as std::ffi::c_int;
    } else {
        (*c).params.mmheight = (*c).driverparams.mmheight;
    }
    if (*c).driverparams.minwidth == 0 {
        (*c).params.minwidth = (*c).params.width;
    } else {
        (*c).params.minwidth = (*c).driverparams.minwidth;
    }
    if (*c).driverparams.minheight == 0 {
        (*c).params.minheight = (*c).params.height;
    } else {
        (*c).params.minheight = (*c).driverparams.minheight;
    }
    if (*c).driverparams.maxwidth == 0 {
        (*c).params.maxwidth = (*c).params.width;
    } else {
        (*c).params.maxwidth = (*c).driverparams.maxwidth;
    }
    if (*c).driverparams.maxheight == 0 {
        (*c).params.maxheight = (*c).params.height;
    } else {
        (*c).params.maxheight = (*c).driverparams.maxheight;
    }
    return 1 as std::ffi::c_int;
}

pub unsafe extern "C" fn aa_init(
    mut driver: *const aa_driver,
    mut defparams: *const aa_hardware_params,
    mut driverdata: *const std::ffi::c_void,
) -> *mut aa_context {
    let mut c: *mut aa_context = 0 as *mut aa_context;
    c = calloc(
        1 as std::ffi::c_int as std::ffi::c_ulong,
        ::core::mem::size_of::<aa_context>() as std::ffi::c_ulong,
    ) as *mut aa_context;
    (*c).driverdata = 0 as *mut std::ffi::c_void;
    (*c).mousedriverdata = 0 as *mut std::ffi::c_void;
    (*c).kbddriverdata = 0 as *mut std::ffi::c_void;
    if c.is_null() {
        return 0 as *mut aa_context;
    }
    if ((*driver).init).expect("non-null function pointer")(
        defparams,
        driverdata,
        &mut (*c).driverparams,
        &mut (*c).driverdata,
    ) == 0
    {
        free(c as *mut std::ffi::c_void);
        return 0 as *mut aa_context;
    }
    (*c).driver = driver;
    (*c).kbddriver = 0 as *const aa_kbddriver;
    (*c).mousedriver = 0 as *const aa_mousedriver;
    (*c).params.supported = (*c).driverparams.supported & (*defparams).supported;
    if !((*defparams).font).is_null() {
        (*c).params.font = (*defparams).font;
    } else {
        (*c).params.font = (*c).driverparams.font;
    }
    if ((*c).params.font).is_null() {
        (*c).params.font = (*defparams).font;
    }
    if ((*c).params.font).is_null() {
        (*c).params.font = &aa_font16;
    }
    if (*c).params.supported == 0 {
        (*c).params.supported = (*c).driverparams.supported;
    }
    (*c).mulx = 2 as std::ffi::c_int;
    (*c).muly = 2 as std::ffi::c_int;
    (*c).cursorx = 0 as std::ffi::c_int;
    (*c).cursory = 0 as std::ffi::c_int;
    (*c).mousex = 0 as std::ffi::c_int;
    (*c).mousey = 0 as std::ffi::c_int;
    (*c).buttons = 0 as std::ffi::c_int;
    (*c).table = 0 as *mut std::ffi::c_ushort;
    (*c).filltable = 0 as *mut std::ffi::c_ushort;
    (*c).parameters = 0 as *mut parameters;
    if (*defparams).width != 0 {
        (*c).params.width = (*defparams).width;
    } else if (*c).driverparams.width != 0 {
        (*c).params.width = (*c).driverparams.width;
    } else if (*defparams).recwidth != 0 {
        (*c).params.recwidth = (*defparams).recwidth;
    } else if (*c).driverparams.recwidth != 0 {
        (*c).params.recwidth = (*c).driverparams.recwidth;
    } else {
        (*c).params.width = 80 as std::ffi::c_int;
    }
    if (*defparams).minwidth > (*c).params.width {
        (*c).params.width = (*defparams).minwidth;
    }
    if (*c).driverparams.minwidth > (*c).params.width {
        (*c).params.width = (*c).driverparams.minwidth;
    }
    if (*defparams).maxwidth != 0 && (*defparams).maxwidth > (*c).params.width {
        (*c).params.width = (*defparams).maxwidth;
    }
    if (*c).driverparams.maxwidth != 0 && (*c).driverparams.maxwidth > (*c).params.width {
        (*c).params.width = (*c).driverparams.maxwidth;
    }
    if (*defparams).height != 0 {
        (*c).params.height = (*defparams).height;
    } else if (*c).driverparams.height != 0 {
        (*c).params.height = (*c).driverparams.height;
    } else if (*defparams).recheight != 0 {
        (*c).params.recheight = (*defparams).recheight;
    } else if (*c).driverparams.recheight != 0 {
        (*c).params.recheight = (*c).driverparams.recheight;
    } else {
        (*c).params.height = 25 as std::ffi::c_int;
    }
    if (*defparams).minheight > (*c).params.height {
        (*c).params.height = (*defparams).minheight;
    }
    if (*c).driverparams.minheight > (*c).params.height {
        (*c).params.height = (*c).driverparams.minheight;
    }
    if (*defparams).maxheight != 0 && (*defparams).maxheight > (*c).params.height {
        (*c).params.height = (*defparams).maxheight;
    }
    if (*c).driverparams.maxheight != 0 && (*c).driverparams.maxheight > (*c).params.height {
        (*c).params.height = (*c).driverparams.maxheight;
    }
    (*c).params.width *= -(1 as std::ffi::c_int);
    (*c).params.height *= -(1 as std::ffi::c_int);
    (*c).params.dimmul = 5.3f64;
    (*c).params.boldmul = 2.7f64;
    if (*c).driverparams.dimmul != 0. {
        (*c).params.dimmul = (*c).driverparams.dimmul;
    }
    if (*c).driverparams.boldmul != 0. {
        (*c).params.boldmul = (*c).driverparams.boldmul;
    }
    if (*defparams).dimmul != 0. {
        (*c).params.dimmul = (*defparams).dimmul;
    }
    if (*defparams).boldmul != 0. {
        (*c).params.boldmul = (*defparams).boldmul;
    }
    (*c).imagebuffer = 0 as *mut std::ffi::c_uchar;
    (*c).textbuffer = 0 as *mut std::ffi::c_uchar;
    (*c).attrbuffer = 0 as *mut std::ffi::c_uchar;
    (*c).resizehandler = None;
    if aa_resize(c) == 0 {
        ((*driver).uninit).expect("non-null function pointer")(c);
        if !((*c).driverdata).is_null() {
            free((*c).driverdata);
        }
        free(c as *mut std::ffi::c_void);
        printf(b"out of memory\n\0" as *const u8 as *const std::ffi::c_char);
        return 0 as *mut aa_context;
    }
    if !(((*defparams).minwidth != 0
        || (*defparams).maxwidth != 0
        || ((*defparams).width == (*c).params.width || (*defparams).width == 0))
        && ((*defparams).minheight != 0
            || (*defparams).maxheight != 0
            || ((*defparams).height == (*c).params.height || (*defparams).height == 0))
        && (if (*defparams).minwidth != 0 {
            ((*defparams).minwidth <= (*c).params.width) as std::ffi::c_int
        } else {
            1
        }) != 0
        && (if (*defparams).minheight != 0 {
            ((*defparams).minheight <= (*c).params.width) as std::ffi::c_int
        } else {
            1
        }) != 0
        && (if (*defparams).maxwidth != 0 {
            ((*defparams).maxwidth >= (*c).params.width) as std::ffi::c_int
        } else {
            1
        }) != 0
        && (if (*defparams).maxheight != 0 {
            ((*defparams).maxheight >= (*c).params.width) as std::ffi::c_int
        } else {
            1
        }) != 0)
    {
        aa_close(c);
        return 0 as *mut aa_context;
    }
    return c;
}
unsafe extern "C" fn aa_invalidate(mut c: *mut aa_context) {
    if !((*c).table).is_null() {
        free((*c).table as *mut std::ffi::c_void);
    }
    if !((*c).filltable).is_null() {
        free((*c).filltable as *mut std::ffi::c_void);
    }
    if !((*c).parameters).is_null() {
        free((*c).parameters as *mut std::ffi::c_void);
    }
    (*c).table = 0 as *mut std::ffi::c_ushort;
    (*c).filltable = 0 as *mut std::ffi::c_ushort;
    (*c).parameters = 0 as *mut parameters;
}

pub unsafe extern "C" fn aa_setfont(mut c: *mut aa_context, mut font: *const aa_font) {
    (*c).params.font = font;
    aa_invalidate(c);
}

pub unsafe extern "C" fn aa_setsupported(mut c: *mut aa_context, mut supported: std::ffi::c_int) {
    (*c).params.supported = (*c).driverparams.supported & supported;
    if (*c).params.supported == 0 {
        (*c).params.supported = (*c).driverparams.supported;
    }
    aa_invalidate(c);
}

pub unsafe extern "C" fn aa_close(mut c: *mut aa_context) {
    if (*c).cursorstate < 0 as std::ffi::c_int && ((*(*c).driver).cursormode).is_some() {
        ((*(*c).driver).cursormode).expect("non-null function pointer")(c, 1 as std::ffi::c_int);
    }
    if !((*c).kbddriver).is_null() {
        aa_uninitkbd(c);
    }
    ((*(*c).driver).uninit).expect("non-null function pointer")(c);
    aa_invalidate(c);
    if !((*c).imagebuffer).is_null() {
        free((*c).imagebuffer as *mut std::ffi::c_void);
    }
    if !((*c).textbuffer).is_null() {
        free((*c).textbuffer as *mut std::ffi::c_void);
    }
    if !((*c).attrbuffer).is_null() {
        free((*c).attrbuffer as *mut std::ffi::c_void);
    }
    if !((*c).driverdata).is_null() {
        free((*c).driverdata);
    }
    free(c as *mut std::ffi::c_void);
}
