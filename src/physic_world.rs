extern crate nalgebra;

use global::PlayerSide;
use global::PlayerSide::*;
use game_constants::*;

use physic_world::nalgebra::base::Vector2;
use vector::VectorOP;

pub const TIMESTEP : usize = 5usize; // calculations per frame

pub const TIMEOUT_MAX : f32 = 2.5f32;

// Gamefeeling relevant constants:
pub const BLOBBY_ANIMATION_SPEED : f32 = 0.5f32;

pub const STANDARD_BALL_ANGULAR_VELOCITY : f32 = 0.1f32;
pub const STANDARD_BALL_HEIGHT : f32 = 269f32 + BALL_RADIUS;


pub struct PhysicWorld {
    ball_hit_by_blobs : [bool; 2],
    blob_positions : [Vector2<f32>; 2],
    ball_position : Vector2<f32>,
    blob_velocities : [Vector2<f32>; 2],
    ball_velocity : Vector2<f32>,

    ball_rotation : f32,
    ball_angular_velocity : f32,
    blobs_animation_states : [f32; 2],
    blobs_animation_speed : [f32; 2],

    is_game_running : bool,
    is_ball_valid : bool,
    last_hit_intensity: f32,
    time_since_ball_out: f32,
}

impl PhysicWorld {

    pub fn new() -> PhysicWorld {
        let mut physic_world = PhysicWorld {
            ball_hit_by_blobs : [false; 2],
            blob_positions : [Vector2::new(0.0f32, 0.0f32); 2],
            ball_position : Vector2::new(0.0f32, 0.0f32),
            blob_velocities : [Vector2::new(0.0f32, 0.0f32); 2],
            ball_velocity : Vector2::new(0.0f32, 0.0f32),

            ball_rotation : 0.0f32,
            ball_angular_velocity : 0.0f32,
            blobs_animation_states : [0.0f32; 2],
            blobs_animation_speed : [0.0f32; 2],

            is_game_running : false,
            is_ball_valid : false,
            last_hit_intensity: 0.0f32,
            time_since_ball_out: 0.0f32,
        };

        physic_world.reset(LeftPlayer);
        physic_world.blobs_animation_speed[LeftPlayer as usize] = 0.0f32;
        physic_world.blobs_animation_speed[RightPlayer as usize] = 0.0f32;

        physic_world
    }

    pub fn get_ball_position(&self) -> Vector2<f32> {
        self.ball_position
    }

    pub fn get_ball_rotation(&self) -> f32 {
        self.ball_rotation
    }

    pub fn get_blob(&self, player: PlayerSide) -> Vector2<f32> {
        self.blob_positions[player as usize]
    }

    pub fn get_blob_state(&self, player: PlayerSide) -> f32 {
        self.blobs_animation_states[player as usize]
    }

    pub fn reset_player(&mut self) {
        self.blob_positions[LeftPlayer as usize] =  Vector2::new(200.0f32, GROUND_PLANE_HEIGHT);
        self.blob_positions[RightPlayer as usize] = Vector2::new(600.0f32, GROUND_PLANE_HEIGHT);
    }

    pub fn reset(&mut self, player: PlayerSide) {
        if player == LeftPlayer
        {
            self.ball_position = Vector2::new(200f32, STANDARD_BALL_HEIGHT);
        }
        else if player == RightPlayer
        {
            self.ball_position = Vector2::new(600f32, STANDARD_BALL_HEIGHT);
        }
        else
        {
            self.ball_position = Vector2::new(400f32, 450f32);
        }

        self.ball_velocity.clear();

        self.ball_rotation = 0.0f32;
        self.ball_angular_velocity = STANDARD_BALL_ANGULAR_VELOCITY;

        self.blobs_animation_states[LeftPlayer as usize] = 0.0f32;
        self.blobs_animation_states[RightPlayer as usize] = 0.0f32;

        self.is_game_running = false;
        self.is_ball_valid = true;

        self.last_hit_intensity = 0.0;
    }

    pub fn get_ball_speed(&self) -> f32 {
        self.ball_velocity.length()
    }

    pub fn player_bottom_ball_collision(&mut self, player : PlayerSide) -> bool {
        let player_index = player.clone() as usize;

        let pos = 		        
            Vector2::new
            (
                self.blob_positions[player_index].x,
                self.blob_positions[player_index].y + BLOBBY_LOWER_SPHERE
            );

        if (pos - self.ball_position).length() <= BALL_RADIUS + BLOBBY_LOWER_RADIUS
        {
            return true;
        }
        false
    }

    pub fn player_top_ball_collision(&mut self, player : PlayerSide) -> bool {
        let player_index = player.clone() as usize;
        let pos = 		        
            Vector2::new
            (
                self.blob_positions[player_index].x,
                self.blob_positions[player_index].y - BLOBBY_UPPER_SPHERE
            );
        if (pos - self.ball_position).length() <= BALL_RADIUS + BLOBBY_UPPER_RADIUS
        {
		    return true;
        }

        false
    }

    pub fn check_blobby_ball_collision(&mut self, player : PlayerSide) {
        let player_index = player.clone() as usize;
        if self.player_bottom_ball_collision(player.clone()) {
            self.last_hit_intensity = (self.ball_velocity - self.blob_velocities[player_index]).length();

            let blobpos = self.blob_positions[player_index];
            let circlepos : Vector2<f32> = Vector2::new(blobpos.x, blobpos.y + BLOBBY_LOWER_SPHERE);
            
            self.ball_velocity = -(self.ball_position - circlepos);
            self.ball_velocity = self.ball_velocity.normalise();
            self.ball_velocity = self.ball_velocity.scale(BALL_COLLISION_VELOCITY);
            self.ball_position += self.ball_velocity;
            self.ball_hit_by_blobs[player_index] = true;
        }
        else if self.player_top_ball_collision(player.clone()) {
            self.last_hit_intensity = (self.ball_velocity - self.blob_velocities[player_index]).length();

            let blobpos = self.blob_positions[player_index];
            let circlepos : Vector2<f32> = Vector2::new(blobpos.x, blobpos.y - BLOBBY_UPPER_SPHERE);

            self.ball_velocity = -(self.ball_position - circlepos);
            self.ball_velocity = self.ball_velocity.normalise();
            self.ball_velocity = self.ball_velocity.scale(BALL_COLLISION_VELOCITY);
            self.ball_position += self.ball_velocity;
            self.ball_hit_by_blobs[player_index] = true;
        }

    }

    pub fn step(&mut self) {
        self.ball_velocity.y += BALL_GRAVITATION;
        self.ball_position += self.ball_velocity;

        if self.is_ball_valid {
            self.check_blobby_ball_collision(LeftPlayer);
            self.check_blobby_ball_collision(RightPlayer);
        }
        else if self.ball_position.y + BALL_RADIUS > 500.0f32 {
            self.ball_velocity = self.ball_velocity.reflect_y().scale_y(0.5f32);
            self.ball_velocity = self.ball_velocity.scale_x(0.55f32);
            self.ball_position.y = 500.0f32 - BALL_RADIUS;
        }

        if self.ball_position.x - BALL_RADIUS <= LEFT_PLANE && self.ball_velocity.x < 0.0
        {
            self.ball_velocity = self.ball_velocity.reflect_x();
            // set the ball's position
            self.ball_position.x = LEFT_PLANE + BALL_RADIUS;
        }
        else if self.ball_position.x + BALL_RADIUS >= RIGHT_PLANE && self.ball_velocity.x > 0.0
        {
            self.ball_velocity = self.ball_velocity.reflect_x();
            // set the ball's position
            self.ball_position.x = RIGHT_PLANE - BALL_RADIUS;
        }
        else if 
        
            self.ball_position.y > NET_SPHERE_POSITION &&
            (self.ball_position.x - NET_POSITION_X).abs() < BALL_RADIUS + NET_RADIUS
        {
            self.ball_velocity = self.ball_velocity.reflect_x();
            // set the ball's position so that it touches the net
            let delta =  
                if self.ball_position.x - NET_POSITION_X > 0.0f32 {
                    BALL_RADIUS + NET_RADIUS 
                } 
                else {
                    -BALL_RADIUS - NET_RADIUS
                };

            self.ball_position.x =  NET_POSITION_X + delta;
        }
        else
        {
            // Net Collisions

            let ball_net_vec : Vector2<f32> = 
                self.ball_position - Vector2::new(NET_POSITION_X, NET_SPHERE_POSITION);

            let ball_net_distance = ball_net_vec.length();

            if ball_net_distance < NET_RADIUS + BALL_RADIUS
            { 
                let vec = self.ball_position - Vector2::new(NET_POSITION_X, NET_SPHERE_POSITION);
                // calculate
                let normal = vec.normalise();
                        
                // normal component of kinetic energy
                let mut perp_ekin = normal.dot_product(&self.ball_velocity);

                perp_ekin *= perp_ekin;
                // parallel component of kinetic energy
                let mut para_ekin = self.ball_velocity.length() * self.ball_velocity.length() - perp_ekin;
                
                // the normal component is damped stronger than the parallel component
                // the values are ~ 0.85² and ca. 0.95², because speed is sqrt(ekin)
                perp_ekin *= 0.7;
                para_ekin *= 0.9;
                
                let nspeed = (perp_ekin + para_ekin).sqrt();
                
                self.ball_velocity = self.ball_velocity.reflect(&normal).normalise().scale(nspeed);
                
                // pushes the ball out of the net
                self.ball_position = 
                    Vector2::new
                    (
                        NET_POSITION_X, 
                        NET_SPHERE_POSITION
                    ) - normal * (NET_RADIUS + BALL_RADIUS);
            }

            // self.ball_velocity = self.ball_velocity.reflect( Vector2( self.ball_position, Vector2 (NET_POSITION_X, temp) ).normalise()).scale(0.75);
        }

        if self.blob_positions[LeftPlayer as usize].x + BLOBBY_LOWER_RADIUS > NET_POSITION_X - NET_RADIUS // Collision with the net
        {
		    self.blob_positions[LeftPlayer as usize].x = NET_POSITION_X - NET_RADIUS - BLOBBY_LOWER_RADIUS;
        }

        if self.blob_positions[RightPlayer as usize].x - BLOBBY_LOWER_RADIUS < NET_POSITION_X + NET_RADIUS
        {
            self.blob_positions[RightPlayer as usize].x = NET_POSITION_X + NET_RADIUS + BLOBBY_LOWER_RADIUS;
        }

        // Collision between blobby and the border
        if self.blob_positions[LeftPlayer as usize].x < LEFT_PLANE {
            self.blob_positions[LeftPlayer as usize].x = LEFT_PLANE;
        }

        if self.blob_positions[RightPlayer as usize].x > RIGHT_PLANE {
            self.blob_positions[RightPlayer as usize].x = RIGHT_PLANE;
        }


        // Velocity Integration
        if self.ball_velocity.x > 0.0 {
            self.ball_rotation += self.ball_angular_velocity * (self.get_ball_speed() / 6.0f32);
        }
        else if self.ball_velocity.x < 0.0 {
            self.ball_rotation -= self.ball_angular_velocity * (self.get_ball_speed() / 6.0f32);
        }
        else {
            self.ball_rotation -= self.ball_angular_velocity;
        }


        // Overflow-Protection
        if self.ball_rotation <= 0f32 {
            self.ball_rotation = 6.25f32 + self.ball_rotation;
        }
        else if self.ball_rotation >= 6.25f32 {
            self.ball_rotation = self.ball_rotation - 6.25f32;
        }

        self.time_since_ball_out = 
            if self.is_ball_valid 
            { 0.0 } 
            else 
            { self.time_since_ball_out + 1.0f32 / 60f32 }
    }

}