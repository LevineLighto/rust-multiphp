# Multi PHP Command Line

A command line tool to help running php script across different versions.

## Is A Personal Project

This tool is a personal project targetted to my own personal computer, so it only runs
PHP versions installed in my PC.

## Background

There are two ways run different version of PHP CLI in my PC,

1.  Pass the whole path (eg: `C:\xampp\php74\php.exe`) everytime I want to run
    a command. It's a very huge pain to do this everytime I want to run artisan command
    on my Laravel project `C:\xampp\php74\php.exe artisan make:model -m WhatDaHeck` (╯°□°）╯︵ ┻━┻
2.  Create an alias. I did this by using Windows .bat files. I made an "alias" folder, 
    register it on my path, and add .bat files there.

I chose the second option. I created 2 .bat files, one for running PHP 7.4 and the other
for running composer using PHP 7.4

PHP 7.4
```batch
@echo off
C:\xampp\php74\php.exe %*
```

Composer running on PHP 7.4
```batch
@echo off
php74 C:\ProgramData\ComposerSetup\bin\composer.phar %*
```

There weren't much problem, until few months later, I had to install PHP 8.2 and PHP 8.3.
And guess what? Yep! I needed to make another .bat files. 4 files to be precise.
1. Alias for PHP 8.2
2. Alias for Composer using PHP 8.2
3. Alias for PHP 8.3
4. Alias for Composer using PHP 8.3

For the lazy me, it was too much hassle to duplicate some (two) .bat files. 
And viola! A new Rust project to handle those alias.

> A new learning opportunity

Or so I thought. But now that I think of it, creating this project requires more effort
than duplicating those .bat files (l|l⚆ᗝ⚆)