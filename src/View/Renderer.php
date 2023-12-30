<?php

declare(strict_types=1);

namespace MetalLE\View;

class Renderer
{
    public function format_date(\DateTimeInterface $dateTime): string
    {
        return str_replace(
            ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'],
            ['Mo', 'Di', 'Mi', 'Do', 'Fr', 'Sa', 'So'],
            $dateTime->format('D. d.m.'),
        );
    }

    public function render(string $file, array $data): string
    {
        $view = $this;

        ob_start();
        include __DIR__ . '/' . $file;
        return $this->minify(ob_get_clean());
    }

    private function minify(string $html): string
    {
        return preg_replace('#(\s)\s+#', '$1', $html);
    }
}