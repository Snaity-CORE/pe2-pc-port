/* Hand-computed checks for the software GTE. */
#include "pc/gte.h"

#include <stdio.h>

static int failures;

#define CHECK(expr, want)                                                           \
    do {                                                                            \
        long long got_ = (long long)(expr);                                         \
        if (got_ != (long long)(want)) {                                            \
            printf("FAIL %s:%d: %s = %lld, want %lld\n", __FILE__, __LINE__, #expr, \
                   got_, (long long)(want));                                        \
            failures++;                                                             \
        }                                                                           \
    } while (0)

static void set_identity_rt(void)
{
    pcgte_write_ctrl(PCGTE_RT0, 0x1000);       /* RT11=1.0, RT12=0 */
    pcgte_write_ctrl(PCGTE_RT1, 0);            /* RT13, RT21 */
    pcgte_write_ctrl(PCGTE_RT2, 0x1000);       /* RT22=1.0, RT23=0 */
    pcgte_write_ctrl(PCGTE_RT3, 0);            /* RT31, RT32 */
    pcgte_write_ctrl(PCGTE_RT4, 0x1000);       /* RT33=1.0 */
}

static void test_rtps(void)
{
    pcgte_reset();
    set_identity_rt();
    pcgte_write_ctrl(PCGTE_TRZ, 1000);
    pcgte_write_ctrl(PCGTE_H, 1000);
    pcgte_write_ctrl(PCGTE_OFX, 160 << 16);
    pcgte_write_ctrl(PCGTE_OFY, 120 << 16);
    pcgte_write_data(PCGTE_VXY0, (100 & 0xFFFF) | ((unsigned)(-50 & 0xFFFF) << 16));
    pcgte_write_data(PCGTE_VZ0, 0);
    pcgte_op(0x4A180001); /* RTPS, sf=1 */
    CHECK(pcgte_read_data(PCGTE_SZ3), 1000);
    CHECK((short)(pcgte_read_data(PCGTE_SXY2) & 0xFFFF), 160 + 100);
    CHECK((short)(pcgte_read_data(PCGTE_SXY2) >> 16), 120 - 50);
    CHECK((int)pcgte_read_data(PCGTE_IR1), 100);
    CHECK((int)pcgte_read_data(PCGTE_IR2), -50);
    CHECK(pcgte_read_ctrl(PCGTE_FLAG), 0);
    /* SXYP reads SXY2. */
    CHECK(pcgte_read_data(PCGTE_SXYP), pcgte_read_data(PCGTE_SXY2));
}

static void test_divide_overflow(void)
{
    pcgte_reset();
    set_identity_rt();
    pcgte_write_ctrl(PCGTE_TRZ, 10);
    pcgte_write_ctrl(PCGTE_H, 1000);   /* H >= 2*SZ3 -> overflow */
    pcgte_op(0x4A180001);
    CHECK((pcgte_read_ctrl(PCGTE_FLAG) >> 17) & 1, 1);
    CHECK(pcgte_read_ctrl(PCGTE_FLAG) >> 31, 1);
}

static void test_nclip_avsz(void)
{
    pcgte_reset();
    /* Counter-clockwise triangle (0,0) (10,0) (0,10): area*2 = 100. */
    pcgte_write_data(PCGTE_SXY0, 0);
    pcgte_write_data(PCGTE_SXY1, 10);
    pcgte_write_data(PCGTE_SXY2, 10u << 16);
    pcgte_op(0x4B400006);
    CHECK((int)pcgte_read_data(PCGTE_MAC0), 100);

    pcgte_write_data(PCGTE_SZ1, 300);
    pcgte_write_data(PCGTE_SZ2, 600);
    pcgte_write_data(PCGTE_SZ3, 900);
    pcgte_write_ctrl(PCGTE_ZSF3, 0x1000 / 3);
    pcgte_op(0x4B58002D);
    /* 1365 * 1800 >> 12 = 599 */
    CHECK(pcgte_read_data(PCGTE_OTZ), 599);
}

static void test_registers(void)
{
    pcgte_reset();
    pcgte_write_data(PCGTE_LZCS, 0x00F00000);
    CHECK(pcgte_read_data(PCGTE_LZCR), 8);
    pcgte_write_data(PCGTE_LZCS, 0xFFF00000);
    CHECK(pcgte_read_data(PCGTE_LZCR), 12);
    pcgte_write_data(PCGTE_IRGB, 0x7FFF);
    CHECK(pcgte_read_data(PCGTE_IR1), 0x1F << 7);
    CHECK(pcgte_read_data(PCGTE_ORGB), 0x7FFF);
    pcgte_write_data(PCGTE_IR1, 0xFFFF8000);
    CHECK((int)pcgte_read_data(PCGTE_IR1), -0x8000);
    pcgte_write_ctrl(PCGTE_H, 0x8000);
    CHECK(pcgte_read_ctrl(PCGTE_H), 0xFFFF8000u); /* sign-extension quirk */
}

static void test_mvmva_sqr_gpf(void)
{
    pcgte_reset();
    set_identity_rt();
    pcgte_write_data(PCGTE_IR1, 3);
    pcgte_write_data(PCGTE_IR2, -4);
    pcgte_write_data(PCGTE_IR3, 5);
    pcgte_op(0x4A400012 | (3 << 15) | (3 << 13) | (1 << 19)); /* RT x IR, no T, sf=1 */
    CHECK((int)pcgte_read_data(PCGTE_IR2), -4);

    pcgte_op(0x4A980428); /* SQR sf=1 lm=1 -> (16 >> 12) = 0 */
    CHECK((int)pcgte_read_data(PCGTE_MAC2), 0);
    CHECK((int)pcgte_read_data(PCGTE_IR3), 0);

    pcgte_write_data(PCGTE_IR3, 5);
    pcgte_op(0x4A900428); /* SQR sf=0 lm=1 */
    CHECK((int)pcgte_read_data(PCGTE_MAC3), 25);
    CHECK((int)pcgte_read_data(PCGTE_IR3), 25);

    pcgte_write_data(PCGTE_IR0, 0x1000);
    pcgte_write_data(PCGTE_IR1, 0x100);
    pcgte_write_data(PCGTE_IR2, 0x200);
    pcgte_write_data(PCGTE_IR3, 0x300);
    pcgte_op(0x4B98003D); /* GPF sf=1 */
    CHECK(pcgte_read_data(PCGTE_RGB2) & 0xFFFFFF, 0x302010);
}

int main(void)
{
    test_rtps();
    test_divide_overflow();
    test_nclip_avsz();
    test_registers();
    test_mvmva_sqr_gpf();
    if (failures) {
        printf("%d check(s) failed\n", failures);
        return 1;
    }
    printf("gte: all checks passed\n");
    return 0;
}
