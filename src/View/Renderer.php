<?php

declare(strict_types=1);

namespace MetalLE\View;

class Renderer
{
    private string $thisYear;

    public function __construct()
    {
        $this->thisYear = date('Y');
    }

    public function format_date(\DateTimeInterface $dateTime): string
    {
        $format = 'D. d.m.';

        if ($dateTime->format('Y') !== $this->thisYear) {
            $format = 'D. d.m. Y';
        }

        return str_replace(
            ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'],
            ['Mo', 'Di', 'Mi', 'Do', 'Fr', 'Sa', 'So'],
            $dateTime->format($format),
        );
    }

    public function render(string $file, array $data): string
    {
        $view = $this;

        ob_start();
        include __DIR__ . '/' . $file;
        return ob_get_clean();
    }
}