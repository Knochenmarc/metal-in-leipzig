<?php

declare(strict_types=1);

namespace MetalLE\Event;

class Image
{
    public readonly string $publicAvifUrl;
    public readonly string $publicJpgUrl;
    public readonly string $hash;
    public readonly ?int $width;
    public readonly ?int $height;
    public readonly ?float $ratio;

    public function __construct(
        public readonly string $remoteUrl
    ) {
        $this->hash          = hash('xxh3', $remoteUrl);
        $this->publicAvifUrl = 'flyer/' . $this->hash . '.avif';
        $this->publicJpgUrl  = 'flyer/' . $this->hash . '.jpg';
    }

    public function setSize(int $width, int $height): void
    {
        $this->width  = $width;
        $this->height = $height;
        $this->ratio  = $width / $height;
    }
}