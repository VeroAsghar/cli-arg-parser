




args: -t 5 -b false
flags: t b


first pass:
    args: 5 -b false
    arg: -t
    flag_name:
    flag_idx

    check for - => true
    remove - from arg
    compare arg with flags => true
    flag_idx = arg_loc

second_pass
    args: -b false
    arg: 5
    flag_name: t
    flag_idx: arg_loc

    check for - => false
    parse flag type from t
    change flag value at flag_idx


