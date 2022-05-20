[M2] (golly 4.0)
#R Flow6
#C A Turing Machine in Flow6.
#C It is called the Flow6 Read-If/Else System, abbreviated to FRieS. I am funny.
#C This one in particular has 3 Registers and 4 Bits of state.
#C 
#C Its simplicity comes with its one instruction design which goes as follows:
#C It first reads a bit from somewhere. We call this bit 'arg'.
#C Then it combines `arg` with its 4 bits of state, to make 5 bits.
#C Then it finds and runs the instructions corresponding to the 5 bits.
#C   The Instructions perform operations on the registers, such as Push/Pull/Flip.
#C   They also set the jump state for the program.
#C   Lastly, they read 'arg' from somewhere..
#C ..And the cycle repeats.
#C 
#C One could say it Reads a bit and uses it to pick between 2 instructions;
#C one for If, and one for Else.
#C Thus, Read-If/Else.
1 0 0 0 2
1 2 0 0 2
1 0 2 0 2
2 1 2 3 3
1 2 0 0 0
2 0 5 0 0
3 0 4 0 6
4 0 7 0 0
5 0 0 0 8
6 0 0 0 9
7 0 0 0 10
8 0 0 0 11
9 0 0 0 12
10 0 0 0 13
11 0 0 0 14
12 0 0 0 15
13 0 0 0 16
1 0 2 2 0
1 2 0 2 0
2 0 18 0 19
1 0 2 0 0
2 0 21 0 0
3 4 20 6 22
1 0 0 2 0
1 0 2 2 2
2 24 25 19 3
2 0 1 0 3
1 2 2 0 0
2 0 28 0 0
2 5 0 0 0
3 26 27 29 30
4 23 31 0 0
2 2 21 3 1
2 2 0 5 0
2 5 21 0 0
2 28 0 0 0
3 33 34 35 36
2 18 24 19 19
2 28 24 21 24
2 21 0 0 0
3 38 39 40 36
4 37 41 0 0
5 0 0 32 42
1 2 2 0 2
2 3 3 0 44
3 4 45 6 22
1 2 2 2 0
2 24 47 19 28
3 20 48 22 29
4 46 49 0 0
2 5 1 24 3
2 2 1 3 3
3 51 52 0 35
2 28 0 44 0
3 54 38 30 40
4 53 55 0 0
5 0 0 50 56
6 0 0 43 57
2 28 19 18 0
3 59 4 30 6
2 1 44 1 2
2 21 5 0 0
3 61 20 62 22
4 60 63 0 0
2 24 18 19 28
2 19 1 19 0
3 65 66 29 22
2 19 1 19 3
2 2 0 3 0
3 68 69 36 30
4 67 70 0 0
5 0 0 64 71
2 25 0 3 0
2 28 5 0 0
3 73 73 74 74
2 1 19 0 19
2 21 2 1 5
2 21 28 0 0
3 76 77 78 78
4 75 79 0 0
2 0 25 0 3
2 0 28 0 21
2 5 28 0 0
3 81 82 29 83
2 24 1 24 0
2 19 3 19 0
3 85 86 22 36
4 84 87 0 0
5 0 0 80 88
6 0 0 72 89
7 0 0 58 90
2 3 0 44 0
3 92 73 40 74
2 47 5 28 24
3 94 0 36 0
4 93 95 0 0
5 0 0 96 0
6 0 0 97 0
7 0 0 98 0
8 0 0 91 99
9 0 0 100 0
10 0 0 101 0
11 0 0 102 0
12 0 0 103 0
13 0 0 104 0
1 1 1 1 0
2 0 0 0 106
1 1 1 0 0
1 1 1 0 1
2 108 106 0 109
3 0 107 0 110
1 0 1 0 0
2 112 109 0 112
2 0 108 0 0
3 0 113 0 114
4 0 111 0 115
2 0 112 0 0
3 0 117 0 0
4 0 118 0 0
5 0 116 0 119
6 0 120 0 0
7 0 121 0 0
8 0 122 0 0
9 0 123 0 0
10 0 124 0 0
11 0 125 0 0
12 0 126 0 0
13 0 127 0 0
1 0 1 0 1
2 0 0 0 129
1 1 1 3 0
1 1 0 0 0
2 131 106 132 109
2 131 109 132 112
3 107 130 133 134
2 0 0 132 129
2 0 0 132 0
1 1 1 0 3
2 138 109 106 112
2 138 108 106 0
3 136 137 139 140
2 106 131 109 132
3 139 140 142 142
2 109 138 112 106
3 142 142 144 144
4 135 141 143 145
2 0 0 106 0
3 147 147 142 142
2 0 0 129 132
3 149 149 144 144
3 144 144 110 133
3 110 133 134 139
4 148 150 151 152
1 1 0 1 0
2 0 154 108 106
2 0 154 131 106
3 144 144 155 156
2 0 129 131 109
2 0 129 138 109
3 110 133 158 159
2 0 109 112 109
2 132 109 138 109
2 0 112 0 108
2 106 112 106 131
3 161 162 163 164
2 132 112 138 108
2 106 0 106 131
2 109 132 109 138
3 166 164 167 168
4 157 160 165 169
2 0 0 138 108
2 154 0 106 131
3 134 139 171 172
2 129 0 109 138
3 140 142 172 174
2 112 106 108 106
3 167 168 168 176
2 112 106 131 106
2 0 109 131 109
3 168 176 178 179
4 173 175 177 180
5 146 153 170 181
3 107 107 110 133
3 130 136 134 139
3 134 139 140 142
3 140 142 142 144
4 183 184 185 186
3 137 147 140 142
3 147 149 142 144
3 142 144 144 110
3 144 110 133 134
4 188 189 190 191
3 142 144 174 155
3 144 110 156 158
3 178 179 162 166
3 162 166 164 167
4 193 194 195 196
3 133 134 159 171
3 139 140 172 172
3 164 167 168 168
3 168 168 176 178
4 198 199 200 201
5 187 192 197 202
2 0 0 0 112
2 112 106 0 154
3 204 168 0 205
2 0 109 0 129
3 168 176 205 207
3 0 110 0 113
3 133 134 139 140
4 206 208 209 210
2 132 109 0 129
2 132 112 0 0
3 178 179 212 213
2 106 112 154 0
2 106 0 154 0
3 162 166 215 216
4 214 217 143 145
3 0 114 0 117
1 5 1 0 0
2 0 154 220 106
3 221 156 161 162
4 219 145 0 222
3 158 159 166 164
3 171 172 167 168
4 151 152 224 225
5 211 218 223 226
2 109 132 129 0
3 164 167 228 228
3 168 168 205 205
4 229 230 151 152
3 176 178 207 212
3 179 162 213 215
4 232 233 185 186
3 172 174 168 176
3 174 155 178 179
4 185 186 235 236
3 156 158 162 166
3 159 171 164 167
4 190 191 238 239
5 231 234 237 240
6 182 203 227 241
3 149 107 144 110
4 243 135 210 143
4 141 148 145 151
3 142 142 174 174
3 176 178 179 162
3 179 162 166 164
4 246 157 247 248
4 160 173 169 177
5 244 245 249 250
4 150 183 152 185
4 184 188 186 190
4 175 193 180 195
4 194 198 196 200
5 252 253 254 255
3 166 164 216 228
3 167 168 228 205
4 257 258 190 191
4 208 214 210 143
3 172 172 168 168
3 174 174 176 178
4 210 143 261 262
3 155 156 179 162
4 145 151 264 224
5 259 260 263 265
4 217 229 145 151
4 230 232 152 185
4 152 185 225 235
4 186 190 236 238
5 267 268 269 270
6 251 256 266 271
3 117 215 114 142
3 117 144 0 110
4 0 273 0 274
3 216 228 142 144
3 228 205 144 110
4 276 277 191 210
3 0 113 0 0
4 0 279 0 0
2 154 0 154 106
2 154 0 132 0
3 139 140 281 282
2 129 0 129 129
2 129 0 108 0
3 142 142 284 285
2 154 154 108 106
2 0 0 108 154
2 0 154 0 154
2 106 132 109 106
3 287 288 289 290
2 129 129 112 109
2 0 0 108 109
2 0 129 154 129
1 1 1 1 1
2 129 108 112 295
3 292 293 294 296
4 283 286 291 297
5 275 278 280 298
3 205 207 133 134
3 212 213 139 140
4 300 301 143 145
3 215 216 142 142
3 228 228 144 144
4 303 304 151 152
2 0 154 106 132
3 144 144 289 306
2 0 129 0 129
2 0 129 129 108
3 110 133 308 309
2 0 154 0 108
2 154 0 106 108
2 0 0 109 0
2 154 106 154 109
3 311 312 313 314
2 0 129 154 112
2 129 0 109 108
2 132 0 106 154
2 129 129 129 112
3 316 317 318 319
4 307 310 315 320
3 134 139 0 281
3 140 142 282 284
2 108 0 295 109
3 313 287 324 289
3 288 292 290 294
4 322 323 325 326
5 302 305 321 327
1 0 0 0 1
1 1 0 1 1
2 329 330 0 330
2 295 0 129 0
1 0 1 1 1
2 0 333 0 154
2 129 0 129 0
3 331 332 334 335
2 154 333 154 129
1 0 0 1 0
2 333 330 338 106
2 154 329 330 333
1 0 0 1 1
2 154 154 341 330
3 337 339 340 342
2 0 154 0 330
2 129 341 341 330
3 344 345 308 289
2 333 333 333 333
2 341 154 0 0
2 333 0 330 341
2 0 0 338 0
3 347 348 349 350
4 336 343 346 351
2 0 129 0 109
2 0 112 0 129
2 154 109 132 106
3 353 155 354 355
2 333 132 109 0
2 154 0 154 0
3 357 358 335 358
2 0 129 0 112
2 0 108 108 108
3 308 289 360 361
2 129 0 112 108
2 108 108 108 108
2 108 108 108 106
2 109 108 109 108
3 363 364 365 366
4 356 359 362 367
5 0 352 0 368
2 295 329 129 0
2 330 295 330 129
2 129 0 333 0
2 333 129 154 129
3 370 371 372 373
2 333 333 129 338
2 329 154 333 341
3 289 375 344 376
2 154 129 330 341
3 0 378 0 335
2 341 333 330 333
2 333 341 333 0
2 154 333 154 330
2 0 0 341 338
3 380 381 382 383
4 374 377 379 384
2 330 295 106 129
2 154 129 330 333
3 386 331 387 334
3 332 337 335 340
2 154 0 0 0
3 390 344 0 308
3 345 347 289 349
4 388 389 391 392
2 112 154 129 132
3 0 317 0 394
2 154 333 106 109
1 1 0 0 1
2 397 154 106 154
2 109 129 106 129
3 396 398 399 289
2 108 109 106 109
2 106 109 106 109
2 109 108 108 0
3 401 366 402 403
2 106 109 106 108
2 108 132 0 0
2 132 0 0 0
3 405 406 407 0
4 395 400 404 408
3 0 353 0 354
3 155 357 355 335
3 0 308 0 360
3 289 363 361 365
4 410 411 412 413
5 385 393 409 414
6 299 328 369 415
3 205 205 110 133
3 207 212 134 139
4 417 418 185 186
3 213 215 140 142
4 420 276 190 191
3 142 144 285 289
3 144 110 306 308
3 293 311 296 313
3 312 316 314 318
4 422 423 424 425
3 133 134 309 0
3 317 313 319 324
4 427 283 428 291
5 419 421 426 429
4 277 300 210 143
4 301 303 145 151
4 286 307 297 315
4 310 322 320 325
5 431 432 433 434
3 339 370 342 372
3 371 289 373 344
3 348 0 350 0
3 378 380 335 382
4 436 437 438 439
3 375 386 376 387
3 381 390 383 0
4 441 336 442 346
3 358 0 358 0
3 317 396 394 399
3 364 401 366 402
3 366 405 403 407
4 444 445 446 447
3 398 0 289 0
3 406 0 0 0
4 449 356 450 362
5 440 443 448 451
4 343 374 351 379
4 377 388 384 391
4 359 395 367 404
4 400 410 408 412
5 453 454 455 456
6 430 435 452 457
7 242 272 416 458
2 108 108 0 0
3 149 0 144 460
2 131 108 132 0
3 133 462 139 140
4 189 461 191 463
3 0 0 460 460
2 0 0 0 329
2 108 109 0 0
3 0 466 460 467
3 460 460 460 460
2 108 108 0 129
2 108 108 0 329
3 460 470 460 471
4 465 468 469 472
4 199 246 201 247
2 0 0 108 108
3 460 460 475 475
2 0 329 108 109
3 460 467 475 477
2 0 0 131 108
2 132 0 138 108
3 479 475 480 475
2 0 129 108 108
3 475 475 475 482
4 476 478 481 483
5 464 473 474 484
2 329 341 333 329
2 341 341 338 329
2 333 109 0 329
1 0 1 1 0
2 489 295 341 112
3 486 487 488 490
2 338 0 154 0
2 0 338 0 154
2 108 106 154 154
3 492 493 364 494
2 108 109 109 0
2 333 108 0 0
2 129 333 341 329
2 108 108 341 329
3 496 497 498 499
2 109 106 0 109
2 154 154 0 154
2 108 295 341 129
2 0 295 0 154
3 501 502 503 504
4 491 495 500 505
2 329 0 129 0
2 109 108 129 330
2 108 108 341 341
3 507 0 508 509
2 109 0 333 341
3 0 0 509 511
2 333 341 129 0
2 341 341 0 0
3 335 0 513 514
2 129 132 106 154
2 333 341 112 132
3 0 516 514 517
4 510 512 515 518
2 333 109 329 341
2 333 109 341 341
2 333 329 333 109
2 338 329 489 295
3 520 521 522 523
2 333 108 338 0
2 154 0 108 108
3 525 289 526 155
2 341 112 333 108
2 109 0 129 333
3 477 528 529 475
2 108 108 109 106
2 154 154 154 154
2 0 109 108 295
2 0 154 0 295
3 531 532 533 534
4 524 527 530 535
3 335 0 317 475
3 0 0 475 313
2 129 330 129 0
2 129 0 333 341
2 0 0 341 341
3 539 514 540 541
2 333 341 129 132
2 106 154 333 341
3 514 543 541 544
4 537 538 542 545
5 506 519 536 546
4 233 257 186 190
3 167 475 228 0
3 475 477 0 466
3 144 460 133 462
3 460 467 460 470
4 549 550 551 552
4 191 210 239 261
3 460 471 460 467
3 475 475 479 475
4 143 555 262 556
5 548 553 554 557
2 341 329 333 109
3 559 559 486 487
2 341 129 333 108
3 561 289 492 289
3 488 490 496 497
3 364 494 501 502
4 560 562 563 564
3 335 0 335 0
2 112 132 0 0
3 0 567 0 0
3 508 509 335 0
3 509 511 0 516
4 566 568 569 570
2 333 109 0 0
3 498 499 572 572
3 503 504 497 289
3 475 475 475 475
2 0 154 108 295
3 475 155 475 576
4 573 574 575 577
3 513 514 335 0
3 514 517 0 0
2 333 0 109 0
3 581 0 581 0
4 579 580 582 0
5 565 571 578 583
6 485 547 558 584
2 0 0 341 0
3 0 0 586 0
2 112 0 0 0
2 333 0 0 0
3 588 0 589 0
4 587 0 590 0
2 341 0 112 0
2 0 0 333 0
3 592 0 593 0
4 0 0 594 0
5 591 0 595 0
3 586 0 588 0
4 0 0 597 0
3 589 0 0 0
4 599 0 0 0
5 598 0 600 0
6 596 0 601 0
4 304 417 152 185
3 407 0 140 460
3 142 460 144 460
4 418 604 186 605
4 323 422 326 424
3 133 462 309 0
4 423 608 425 428
5 603 606 607 609
2 108 154 0 154
3 0 0 611 0
3 0 289 0 289
2 108 106 0 154
2 109 0 129 0
2 109 108 129 0
3 614 615 614 616
3 0 289 358 289
4 612 613 617 618
4 566 0 566 0
3 614 616 289 335
2 106 109 154 129
2 154 129 154 129
3 622 289 623 289
3 289 335 289 335
3 623 289 623 289
4 621 624 625 626
5 619 620 627 620
4 389 436 392 438
4 437 441 439 442
4 411 444 413 446
4 445 449 447 450
5 629 630 631 632
4 625 626 625 626
5 634 620 634 620
6 610 628 633 635
7 585 602 636 0
2 333 341 129 338
3 289 335 331 638
2 329 154 129 0
3 334 640 289 335
4 0 639 0 641
2 129 341 341 341
3 344 643 0 0
4 0 644 0 0
5 0 642 0 645
2 0 0 106 154
3 314 647 371 289
2 341 341 341 341
3 373 344 378 649
3 541 541 649 649
4 648 0 650 651
3 0 289 0 331
2 0 333 341 330
2 341 341 333 341
2 330 330 330 330
3 541 654 655 656
4 0 653 651 657
3 649 649 0 0
3 0 204 0 0
4 659 659 0 660
2 341 341 341 333
2 341 330 341 330
3 662 663 623 289
2 333 341 333 341
2 330 330 330 154
2 129 329 129 129
2 154 0 341 341
3 665 666 667 668
2 154 129 106 109
2 109 112 106 129
3 670 155 671 355
2 129 329 109 108
2 106 129 154 129
3 673 674 394 623
4 664 669 672 675
5 652 658 661 676
3 623 289 623 311
2 154 112 108 108
2 112 108 108 108
3 335 679 680 364
3 679 364 0 0
3 364 364 0 0
4 678 681 682 683
5 0 684 0 0
6 646 677 0 685
3 335 314 638 371
2 333 154 132 154
3 647 0 688 0
2 333 129 330 333
2 330 333 154 0
3 376 690 665 691
2 0 154 341 154
3 693 0 0 0
4 687 689 692 694
5 695 0 600 0
4 639 648 641 650
4 0 0 651 651
4 644 659 0 0
4 659 664 660 672
5 697 698 699 700
3 364 364 364 364
4 702 702 683 683
5 703 703 0 0
2 108 108 109 108
3 364 364 364 705
2 108 108 108 109
3 364 707 0 308
2 108 106 108 106
2 0 154 341 330
3 709 366 710 540
4 702 706 708 711
3 707 709 402 709
2 108 106 108 132
3 402 709 402 714
3 402 709 387 710
2 106 108 132 0
1 5 5 0 0
2 0 0 718 0
3 717 0 541 719
4 713 715 716 720
3 289 335 114 616
2 129 341 0 0
3 0 723 0 0
4 0 722 0 724
3 623 289 622 614
2 0 338 489 106
2 333 154 329 330
2 154 0 330 341
3 727 312 728 729
2 330 333 154 129
2 341 330 0 154
2 154 129 108 109
3 731 732 733 155
2 397 154 108 154
2 489 106 333 154
2 106 108 154 0
3 735 358 736 737
4 726 730 734 738
5 712 721 725 739
6 696 701 704 740
3 308 710 0 289
2 329 330 397 154
2 330 341 330 341
1 5 0 5 0
2 108 154 745 154
2 341 341 330 341
3 743 744 746 747
3 0 114 0 0
2 333 330 0 0
3 750 514 0 0
4 742 748 749 751
5 0 752 0 0
6 0 753 0 0
7 686 741 0 754
4 653 687 657 692
4 689 0 694 0
2 106 333 295 109
3 673 758 394 623
4 669 599 759 0
5 756 757 760 0
2 108 0 0 0
3 366 717 762 0
4 763 0 0 0
3 475 475 541 541
4 765 765 465 465
5 764 0 766 766
2 0 154 108 132
3 768 335 541 372
2 132 129 0 129
3 623 289 770 289
4 769 626 465 771
5 634 620 772 620
6 761 635 767 773
2 341 341 341 0
2 333 0 341 341
3 775 514 776 541
3 514 514 541 541
3 514 514 0 0
4 777 778 779 779
4 778 778 779 779
5 780 781 0 0
2 341 333 0 0
3 783 289 541 693
4 778 784 779 779
4 566 0 599 0
5 785 786 0 0
6 782 787 0 0
7 774 0 788 0
8 459 637 755 789
9 790 0 0 0
10 791 0 0 0
11 792 0 0 0
12 793 0 0 0
13 794 0 0 0
14 17 105 128 795
