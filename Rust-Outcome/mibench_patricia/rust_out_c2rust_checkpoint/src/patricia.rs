extern "C" {
    fn malloc(__size: size_t) -> *mut core::ffi::c_void;
    fn free(__ptr: *mut core::ffi::c_void);
    fn bcopy(
        __src: *const core::ffi::c_void,
        __dest: *mut core::ffi::c_void,
        __n: size_t,
    );
    fn bit(i: core::ffi::c_int, key: core::ffi::c_ulong) -> core::ffi::c_ulong;
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ptree_mask {
    pub pm_mask: core::ffi::c_ulong,
    pub pm_data: *mut core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ptree {
    pub p_key: core::ffi::c_ulong,
    pub p_m: *mut ptree_mask,
    pub p_mlen: core::ffi::c_uchar,
    pub p_b: core::ffi::c_char,
    pub p_left: *mut ptree,
    pub p_right: *mut ptree,
}
unsafe extern "C" fn insertR(
    mut h: *mut ptree,
    mut n: *mut ptree,
    mut d: core::ffi::c_int,
    mut p: *mut ptree,
) -> *mut ptree {
    if (*h).p_b as core::ffi::c_int >= d
        || (*h).p_b as core::ffi::c_int <= (*p).p_b as core::ffi::c_int
    {
        (*n).p_b = d as core::ffi::c_char;
        (*n).p_left = if bit(d, (*n).p_key) != 0 { h } else { n };
        (*n).p_right = if bit(d, (*n).p_key) != 0 { n } else { h };
        return n;
    }
    if bit((*h).p_b as core::ffi::c_int, (*n).p_key) != 0 {
        (*h).p_right = insertR((*h).p_right, n, d, h);
    } else {
        (*h).p_left = insertR((*h).p_left, n, d, h);
    }
    return h;
}
#[no_mangle]
pub unsafe extern "C" fn pat_insert(
    mut n: *mut ptree,
    mut head: *mut ptree,
) -> *mut ptree {
    let mut t: *mut ptree = 0 as *mut ptree;
    let mut buf: *mut ptree_mask = 0 as *mut ptree_mask;
    let mut pm: *mut ptree_mask = 0 as *mut ptree_mask;
    let mut i: core::ffi::c_int = 0;
    let mut copied: core::ffi::c_int = 0;
    if head.is_null() || n.is_null() || ((*n).p_m).is_null() {
        return 0 as *mut ptree;
    }
    (*n).p_key &= (*(*n).p_m).pm_mask;
    t = head;
    loop {
        i = (*t).p_b as core::ffi::c_int;
        t = if bit((*t).p_b as core::ffi::c_int, (*n).p_key) != 0 {
            (*t).p_right
        } else {
            (*t).p_left
        };
        if !(i < (*t).p_b as core::ffi::c_int) {
            break;
        }
    }
    if (*n).p_key == (*t).p_key {
        i = 0 as core::ffi::c_int;
        while i < (*t).p_mlen as core::ffi::c_int {
            if (*(*n).p_m).pm_mask == (*((*t).p_m).offset(i as isize)).pm_mask {
                let ref mut fresh0 = (*((*t).p_m).offset(i as isize)).pm_data;
                *fresh0 = (*(*n).p_m).pm_data;
                free((*n).p_m as *mut core::ffi::c_void);
                free(n as *mut core::ffi::c_void);
                n = 0 as *mut ptree;
                return t;
            }
            i += 1;
        }
        buf = malloc(
            (::core::mem::size_of::<ptree_mask>() as size_t)
                .wrapping_mul(
                    ((*t).p_mlen as core::ffi::c_int + 1 as core::ffi::c_int) as size_t,
                ),
        ) as *mut ptree_mask;
        copied = 0 as core::ffi::c_int;
        i = 0 as core::ffi::c_int;
        pm = buf;
        while i < (*t).p_mlen as core::ffi::c_int {
            if (*(*n).p_m).pm_mask > (*((*t).p_m).offset(i as isize)).pm_mask {
                bcopy(
                    ((*t).p_m).offset(i as isize) as *const core::ffi::c_void,
                    pm as *mut core::ffi::c_void,
                    ::core::mem::size_of::<ptree_mask>() as size_t,
                );
                i += 1;
            } else {
                bcopy(
                    (*n).p_m as *const core::ffi::c_void,
                    pm as *mut core::ffi::c_void,
                    ::core::mem::size_of::<ptree_mask>() as size_t,
                );
                (*(*n).p_m).pm_mask = 0xffffffff as core::ffi::c_ulong;
                copied = 1 as core::ffi::c_int;
            }
            pm = pm.offset(1);
        }
        if copied == 0 {
            bcopy(
                (*n).p_m as *const core::ffi::c_void,
                pm as *mut core::ffi::c_void,
                ::core::mem::size_of::<ptree_mask>() as size_t,
            );
        }
        free((*n).p_m as *mut core::ffi::c_void);
        free(n as *mut core::ffi::c_void);
        n = 0 as *mut ptree;
        (*t).p_mlen = ((*t).p_mlen).wrapping_add(1);
        free((*t).p_m as *mut core::ffi::c_void);
        (*t).p_m = buf;
        return t;
    }
    i = 1 as core::ffi::c_int;
    while i < 32 as core::ffi::c_int && bit(i, (*n).p_key) == bit(i, (*t).p_key) {
        i += 1;
    }
    if bit((*head).p_b as core::ffi::c_int, (*n).p_key) != 0 {
        (*head).p_right = insertR((*head).p_right, n, i, head);
    } else {
        (*head).p_left = insertR((*head).p_left, n, i, head);
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn pat_remove(
    mut n: *mut ptree,
    mut head: *mut ptree,
) -> core::ffi::c_int {
    let mut p: *mut ptree = 0 as *mut ptree;
    let mut g: *mut ptree = 0 as *mut ptree;
    let mut pt: *mut ptree = 0 as *mut ptree;
    let mut pp: *mut ptree = 0 as *mut ptree;
    let mut t: *mut ptree = 0 as *mut ptree;
    let mut buf: *mut ptree_mask = 0 as *mut ptree_mask;
    let mut pm: *mut ptree_mask = 0 as *mut ptree_mask;
    let mut i: core::ffi::c_int = 0;
    if n.is_null() || ((*n).p_m).is_null() || t.is_null() {
        return 0 as core::ffi::c_int;
    }
    t = head;
    p = t;
    g = p;
    loop {
        i = (*t).p_b as core::ffi::c_int;
        g = p;
        p = t;
        t = if bit((*t).p_b as core::ffi::c_int, (*n).p_key) != 0 {
            (*t).p_right
        } else {
            (*t).p_left
        };
        if !(i < (*t).p_b as core::ffi::c_int) {
            break;
        }
    }
    if (*t).p_key != (*n).p_key {
        return 0 as core::ffi::c_int;
    }
    if (*t).p_mlen as core::ffi::c_int == 1 as core::ffi::c_int {
        if (*t).p_b as core::ffi::c_int == 0 as core::ffi::c_int {
            return 0 as core::ffi::c_int;
        }
        if (*(*t).p_m).pm_mask != (*(*n).p_m).pm_mask {
            return 0 as core::ffi::c_int;
        }
        pt = p;
        pp = pt;
        loop {
            i = (*pt).p_b as core::ffi::c_int;
            pp = pt;
            pt = if bit((*pt).p_b as core::ffi::c_int, (*p).p_key) != 0 {
                (*pt).p_right
            } else {
                (*pt).p_left
            };
            if !(i < (*pt).p_b as core::ffi::c_int) {
                break;
            }
        }
        if bit((*pp).p_b as core::ffi::c_int, (*p).p_key) != 0 {
            (*pp).p_right = t;
        } else {
            (*pp).p_left = t;
        }
        if bit((*g).p_b as core::ffi::c_int, (*n).p_key) != 0 {
            (*g).p_right = if bit((*p).p_b as core::ffi::c_int, (*n).p_key) != 0 {
                (*p).p_left
            } else {
                (*p).p_right
            };
        } else {
            (*g).p_left = if bit((*p).p_b as core::ffi::c_int, (*n).p_key) != 0 {
                (*p).p_left
            } else {
                (*p).p_right
            };
        }
        if !((*(*t).p_m).pm_data).is_null() {
            free((*(*t).p_m).pm_data);
        }
        free((*t).p_m as *mut core::ffi::c_void);
        if t != p {
            (*t).p_key = (*p).p_key;
            (*t).p_m = (*p).p_m;
            (*t).p_mlen = (*p).p_mlen;
        }
        free(p as *mut core::ffi::c_void);
        return 1 as core::ffi::c_int;
    }
    i = 0 as core::ffi::c_int;
    while i < (*t).p_mlen as core::ffi::c_int {
        if (*(*n).p_m).pm_mask == (*((*t).p_m).offset(i as isize)).pm_mask {
            break;
        }
        i += 1;
    }
    if i >= (*t).p_mlen as core::ffi::c_int {
        return 0 as core::ffi::c_int;
    }
    buf = malloc(
        (::core::mem::size_of::<ptree_mask>() as size_t)
            .wrapping_mul(
                ((*t).p_mlen as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
            ),
    ) as *mut ptree_mask;
    i = 0 as core::ffi::c_int;
    pm = buf;
    while i < (*t).p_mlen as core::ffi::c_int {
        if (*(*n).p_m).pm_mask != (*((*t).p_m).offset(i as isize)).pm_mask {
            let fresh1 = pm;
            pm = pm.offset(1);
            bcopy(
                ((*t).p_m).offset(i as isize) as *const core::ffi::c_void,
                fresh1 as *mut core::ffi::c_void,
                ::core::mem::size_of::<ptree_mask>() as size_t,
            );
        }
        i += 1;
    }
    (*t).p_mlen = ((*t).p_mlen).wrapping_sub(1);
    free((*t).p_m as *mut core::ffi::c_void);
    (*t).p_m = buf;
    return 1 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pat_search(
    mut key: core::ffi::c_ulong,
    mut head: *mut ptree,
) -> *mut ptree {
    let mut p: *mut ptree = 0 as *mut ptree;
    let mut t: *mut ptree = head;
    let mut i: core::ffi::c_int = 0;
    if t.is_null() {
        return 0 as *mut ptree;
    }
    loop {
        if (*t).p_key == key & (*(*t).p_m).pm_mask {
            p = t;
        }
        i = (*t).p_b as core::ffi::c_int;
        t = if bit((*t).p_b as core::ffi::c_int, key) != 0 {
            (*t).p_right
        } else {
            (*t).p_left
        };
        if !(i < (*t).p_b as core::ffi::c_int) {
            break;
        }
    }
    return if (*t).p_key == key & (*(*t).p_m).pm_mask { t } else { p };
}
