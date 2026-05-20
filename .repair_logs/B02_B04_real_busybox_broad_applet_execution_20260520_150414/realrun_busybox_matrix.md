| testcase | classification | command | exit | entered_umode | success_emitted | legacy_fallback_used | syscalls | stdout_len | elf_sha256 |
| --- | --- | --- | ---: | --- | --- | --- | ---: | ---: | --- |
| busybox_true | REAL-RUN | `busybox true` | 0 | True | True | False | 3 | 0 | `bc09c5ae5eabf091d0ded0208a17a1173725f10d8b79264d896d09c9553c68de` |
| busybox_echo_independent | REAL-RUN | `busybox echo #### independent command test` | 0 | True | True | False | 6 | 30 | `bc09c5ae5eabf091d0ded0208a17a1173725f10d8b79264d896d09c9553c68de` |
| busybox_pwd | REAL-RUN | `busybox pwd` | 0 | True | True | False | 8 | 6 | `bc09c5ae5eabf091d0ded0208a17a1173725f10d8b79264d896d09c9553c68de` |
| busybox_ls | REAL-RUN | `busybox ls` | 0 | True | True | False | 20 | 70 | `bc09c5ae5eabf091d0ded0208a17a1173725f10d8b79264d896d09c9553c68de` |
| busybox_cat_test_txt | REAL-RUN | `busybox cat test.txt` | 0 | True | True | False | 7 | 12 | `bc09c5ae5eabf091d0ded0208a17a1173725f10d8b79264d896d09c9553c68de` |
| busybox_touch_test_txt | REAL-RUN | `busybox touch test.txt` | 0 | True | True | False | 4 | 0 | `bc09c5ae5eabf091d0ded0208a17a1173725f10d8b79264d896d09c9553c68de` |
| busybox_rm_test_txt | REAL-RUN | `busybox rm test.txt` | 0 | True | True | False | 6 | 0 | `bc09c5ae5eabf091d0ded0208a17a1173725f10d8b79264d896d09c9553c68de` |
| busybox_mkdir_test_dir | REAL-RUN | `busybox mkdir test_dir` | 0 | True | True | False | 4 | 0 | `bc09c5ae5eabf091d0ded0208a17a1173725f10d8b79264d896d09c9553c68de` |
| busybox_rmdir_test | REAL-RUN | `busybox rmdir test` | 0 | True | True | False | 4 | 0 | `bc09c5ae5eabf091d0ded0208a17a1173725f10d8b79264d896d09c9553c68de` |
| busybox_mv_test_dir_test | REAL-RUN | `busybox mv test_dir test` | 0 | True | True | False | 5 | 0 | `bc09c5ae5eabf091d0ded0208a17a1173725f10d8b79264d896d09c9553c68de` |
| busybox_cp_busybox_cmd_bak | REAL-RUN | `busybox cp busybox_cmd.txt busybox_cmd.bak` | 0 | True | True | False | 13 | 0 | `bc09c5ae5eabf091d0ded0208a17a1173725f10d8b79264d896d09c9553c68de` |
| busybox_rm_busybox_cmd_bak | REAL-RUN | `busybox rm busybox_cmd.bak` | 0 | True | True | False | 6 | 0 | `bc09c5ae5eabf091d0ded0208a17a1173725f10d8b79264d896d09c9553c68de` |
| busybox_stat_test_txt | REAL-RUN | `busybox stat test.txt` | 0 | True | True | False | 17 | 339 | `bc09c5ae5eabf091d0ded0208a17a1173725f10d8b79264d896d09c9553c68de` |
| busybox_find_busybox_cmd | REAL-RUN | `busybox find -name busybox_cmd.txt` | 0 | True | True | False | 16 | 18 | `bc09c5ae5eabf091d0ded0208a17a1173725f10d8b79264d896d09c9553c68de` |
| busybox_wc_test_txt | REAL-RUN | `busybox wc test.txt` | 0 | True | True | False | 11 | 39 | `bc09c5ae5eabf091d0ded0208a17a1173725f10d8b79264d896d09c9553c68de` |
| busybox_head_test_txt | REAL-RUN | `busybox head test.txt` | 0 | True | True | False | 11 | 12 | `bc09c5ae5eabf091d0ded0208a17a1173725f10d8b79264d896d09c9553c68de` |
| busybox_tail_test_txt | REAL-RUN | `busybox tail test.txt` | 0 | True | True | False | 12 | 12 | `bc09c5ae5eabf091d0ded0208a17a1173725f10d8b79264d896d09c9553c68de` |
| busybox_sort_test_txt | REAL-RUN | `busybox sort test.txt` | 0 | True | False | False | 11 | 12 | `bc09c5ae5eabf091d0ded0208a17a1173725f10d8b79264d896d09c9553c68de` |
| busybox_uniq_test_txt | REAL-RUN | `busybox uniq test.txt` | 0 | True | False | False | 11 | 12 | `bc09c5ae5eabf091d0ded0208a17a1173725f10d8b79264d896d09c9553c68de` |
| busybox_grep_hello_cmd | REAL-RUN | `busybox grep hello busybox_cmd.txt` | 0 | True | True | False | 12 | 22 | `bc09c5ae5eabf091d0ded0208a17a1173725f10d8b79264d896d09c9553c68de` |
| busybox_cut_c3_test_txt | REAL-RUN | `busybox cut -c 3 test.txt` | 0 | True | True | False | 11 | 2 | `bc09c5ae5eabf091d0ded0208a17a1173725f10d8b79264d896d09c9553c68de` |
| busybox_od_test_txt | REAL-RUN | `busybox od test.txt` | 0 | True | True | False | 13 | 58 | `bc09c5ae5eabf091d0ded0208a17a1173725f10d8b79264d896d09c9553c68de` |
| busybox_hexdump_c_test_txt | REAL-RUN | `busybox hexdump -C test.txt` | 0 | True | True | False | 13 | 84 | `bc09c5ae5eabf091d0ded0208a17a1173725f10d8b79264d896d09c9553c68de` |
| busybox_md5sum_test_txt | REAL-RUN | `busybox md5sum test.txt` | 0 | True | True | False | 11 | 43 | `bc09c5ae5eabf091d0ded0208a17a1173725f10d8b79264d896d09c9553c68de` |
| busybox_strings_test_txt | REAL-RUN | `busybox strings test.txt` | 0 | True | True | False | 11 | 12 | `bc09c5ae5eabf091d0ded0208a17a1173725f10d8b79264d896d09c9553c68de` |
| busybox_basename_aaa_bbb | REAL-RUN | `busybox basename /aaa/bbb` | 0 | True | True | False | 4 | 4 | `bc09c5ae5eabf091d0ded0208a17a1173725f10d8b79264d896d09c9553c68de` |
| busybox_dirname_aaa_bbb | REAL-RUN | `busybox dirname /aaa/bbb` | 0 | True | True | False | 5 | 5 | `bc09c5ae5eabf091d0ded0208a17a1173725f10d8b79264d896d09c9553c68de` |
| busybox_date | REAL-RUN | `busybox date` | 0 | True | True | False | 7 | 29 | `bc09c5ae5eabf091d0ded0208a17a1173725f10d8b79264d896d09c9553c68de` |
| busybox_cal | REAL-RUN | `busybox cal` | 0 | True | True | False | 16 | 165 | `bc09c5ae5eabf091d0ded0208a17a1173725f10d8b79264d896d09c9553c68de` |
| busybox_df | REAL-RUN | `busybox df` | 0 | True | True | False | 13 | 137 | `bc09c5ae5eabf091d0ded0208a17a1173725f10d8b79264d896d09c9553c68de` |
| busybox_du | REAL-RUN | `busybox du` | 0 | True | True | False | 17 | 4 | `bc09c5ae5eabf091d0ded0208a17a1173725f10d8b79264d896d09c9553c68de` |
| busybox_dmesg | REAL-RUN | `busybox dmesg` | 0 | True | True | False | 7 | 0 | `bc09c5ae5eabf091d0ded0208a17a1173725f10d8b79264d896d09c9553c68de` |
| busybox_ps | REAL-RUN | `busybox ps` | 0 | True | True | False | 16 | 29 | `bc09c5ae5eabf091d0ded0208a17a1173725f10d8b79264d896d09c9553c68de` |
| busybox_free | REAL-RUN | `busybox free` | 0 | True | True | False | 15 | 260 | `bc09c5ae5eabf091d0ded0208a17a1173725f10d8b79264d896d09c9553c68de` |
| busybox_uptime | REAL-RUN | `busybox uptime` | 0 | True | True | False | 8 | 62 | `bc09c5ae5eabf091d0ded0208a17a1173725f10d8b79264d896d09c9553c68de` |
| busybox_uname | REAL-RUN | `busybox uname` | 0 | True | True | False | 6 | 13 | `bc09c5ae5eabf091d0ded0208a17a1173725f10d8b79264d896d09c9553c68de` |
| busybox_which_ls | REAL-RUN | `busybox which ls` | 0 | True | True | False | 10 | 8 | `bc09c5ae5eabf091d0ded0208a17a1173725f10d8b79264d896d09c9553c68de` |
| busybox_false | REAL-RUN | `busybox false` | 1 | True | True | False | 3 | 0 | `bc09c5ae5eabf091d0ded0208a17a1173725f10d8b79264d896d09c9553c68de` |
| busybox_sleep_1 | REAL-RUN | `busybox sleep 1` | 0 | True | True | False | 4 | 0 | `bc09c5ae5eabf091d0ded0208a17a1173725f10d8b79264d896d09c9553c68de` |
