#include <mruby/compile.h>
#include <mruby/value.h>

mrb_value tmrb_nil_value() {
    return mrb_nil_value();
}

mrb_aspec TMRB_ARGS_REQ(uint32_t count) {
    return MRB_ARGS_REQ(count);
}
