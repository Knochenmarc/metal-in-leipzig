<?php

declare(strict_types=1);

namespace MetalLE\Event;

class Image
{
    public readonly string $publicUrl;
    public readonly string $hash;
    public readonly ?int $width;
    public readonly ?int $height;
    public readonly ?float $ratio;

    public function __construct(
        public readonly string $remoteUrl
    ) {
        $this->hash      = hash('xxh3', $remoteUrl);
        $this->publicUrl = 'flyer/' . $this->hash . '.avif';
    }

    public function setSize(int $width, int $height): void
    {
        $this->width  = $width;
        $this->height = $height;
        $this->ratio  = $width / $height;
    }
}