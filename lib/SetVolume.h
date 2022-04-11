#pragma once

#ifdef SETVOLUME_EXPORTS
#define SETVOLUME_API __declspec(dllexport)
#else
#define SETVOLUME_API __declspec(dllimport)
#endif

extern "C" SETVOLUME_API void set_volume(float a);

extern "C" SETVOLUME_API void set_mute(bool a);

extern "C" SETVOLUME_API float get_volume();
