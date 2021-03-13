#pragma once

#include "bsp.h"
#include "rkg.h"
#include "stats.h"
#include "vec4.h"

struct wheel {
        u8 idx;
        struct bsp_wheel bsp_wheel;
        struct vec3 pos;
        f32 down;
        struct vec3 last_pos_rel;
};

struct player {
        struct rkg rkg;
        struct stats stats;
        struct bsp bsp;
        bool hop;
        struct vec3 hop_dir;
        f32 turn;
        f32 turn_rot_z_inc;
        bool wheelie;
        u8 wheelie_frame;
        f32 wheelie_rot;
        f32 wheelie_rot_dec;
        bool ground;
        struct vec3 next_top;
        struct vec3 top;
        struct vec3 dir;
        struct vec3 dir_diff;
        f32 start_boost_charge;
        f32 standstill_boost_rot;
        u8 mt_boost;
        struct vec3 inv_inertia_tensor;
        struct vec3 pos;
        f32 normal_acceleration;
        struct vec3 speed0;
        f32 soft_speed_limit;
        f32 speed1_norm;
        struct vec3 speed1;
        struct vec3 speed;
        struct vec3 normal_rot_vec;
        struct vec3 rot_vec0;
        f32 turn_rot_z;
        struct vec4 rot;
        struct vec4 rot2;
        struct wheel wheels[2];
};

void player_init(struct player *player, struct rkg rkg, struct stats stats, struct bsp bsp);

void player_update(struct player *player, u32 frame);
