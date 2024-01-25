"""
ermitteln
~~~~~~~~~
A thin-wrapper around Rust image_hasher to hash images and compare them.

:copyright: (c) 2024-present noaione
:license: MIT, see LICENSE for more details.
"""

def hash_image(image: bytes) -> str:
    """Hashes an image and returns the hash as a base64 encoded string

    The hash is calculated using the pHash (DCT) algorithm
    with combination of Double Gradient Hashing for the best results.

    We only support JPEG images for now.

    Parameters
    ----------
    image: :class:`bytes`
        The image to hash

    Returns
    -------
    :class:`str`
        The base64 encoded hash of the image

    Raises
    ------
    ValueError
        If the image is not a valid JPEG image
    """
    ...

def hamming_distance(hash1: str, hash2: str) -> int:
    """Calculate the hamming distance between two hashes

    Parameters
    ----------
    hash1: :class:`str`
        The first hash or base hash to compare
    hash2: :class:`str`
        The second hash or comparison hash to compare

    Returns
    -------
    :class:`int`
        The hamming distance between the two hashes

    Raises
    ------
    ValueError
        If the hashes are not valid base64 encoded strings
    """

    ...
