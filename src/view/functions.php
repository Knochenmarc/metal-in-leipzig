<?php

declare(strict_types=1);

function format_date(DateTimeInterface $dateTime):string
{
    return str_replace(
        ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'],
        ['Mo', 'Di', 'Mi', 'Do', 'Fr', 'Sa', 'So'],
        $dateTime->format('D. d.m.'),
    );
}