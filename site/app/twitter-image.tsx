import {
  createSocialImage,
  socialImageAlt,
  socialImageContentType,
  socialImageSize,
} from "@/lib/social-image";

export const alt = socialImageAlt;
export const size = socialImageSize;
export const contentType = socialImageContentType;
export const dynamic = "force-static";
// The card is drawn in lib/social-image.tsx. Next derives the ?hash in the
// image URL from this file, so change this note when the card changes to make
// link previews refetch it. Card revision: 2026-10-14 hero redesign.

export default function TwitterImage() {
  return createSocialImage();
}
