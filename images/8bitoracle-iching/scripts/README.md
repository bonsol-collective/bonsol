# I Ching Oracle Scripts

This directory contains scripts for building, deploying, and executing the I Ching Oracle ZK program.

## Prerequisites

1. Install `bonsol` and ensure it's in your PATH
2. Install Rust and Cargo (https://rustup.rs)
3. Configure your AWS credentials in environment variables
4. Run all commands from the project root directory (`~/forked-projects/bonsol` or equivalent)

## Script Order and Purpose

1. `01-build.sh`: Builds both the Rust program and the ZK program
   - First compiles the I Ching Rust code in `images/8bitoracle-iching`
   - Then builds the ZK program using `bonsol build`
   - Handles directory changes automatically

2. `02-deploy.sh`: Deploys the program to S3
   - Requires successful completion of the build step
   - Uses AWS credentials to upload to S3

3. `03-generate-input.sh`: Generates input for program execution
   - Uses the Image ID from the deploy step

4. `04-execute.sh`: Executes the program and displays results
   - Uses the generated input to run the program
   - Displays the I Ching hexagram result

## Environment Setup

Create a `.env` file in the `images/8bitoracle-iching` directory:

```env
# AWS Credentials
AWS_ACCESS_KEY_ID=your_access_key_here
AWS_SECRET_ACCESS_KEY=your_secret_key_here
AWS_REGION=us-east-1

# S3 Configuration
BUCKET=8bitoracle
# Optional: S3_ENDPOINT=https://custom.s3.endpoint
```

## Usage

From the project root directory:

```bash
# 1. Make scripts executable (only needed once)
chmod +x images/8bitoracle-iching/scripts/*.sh

# 2. Source your environment variables
source images/8bitoracle-iching/.env

# 3. Build the program (this handles both Rust and ZK builds)
images/8bitoracle-iching/scripts/01-build.sh

# 4. Deploy to S3
images/8bitoracle-iching/scripts/02-deploy.sh

# 5. Generate input using the Image ID from step 4
images/8bitoracle-iching/scripts/03-generate-input.sh <image_id>

# 6. Execute the program
images/8bitoracle-iching/scripts/04-execute.sh
```

## Important Notes

1. **Working Directory**: 
   - Run all scripts from the project root directory
   - The build script will automatically change to the correct directories for each step
   - You don't need to manually change directories

2. **Environment Variables**: 
   - Use `source` or `.` to load environment variables: `. images/8bitoracle-iching/.env`
   - Direct execution (`./env.sh`) won't persist variables in your shell
   - Environment variables must be loaded before running deploy

3. **Build Process**:
   - The build script (`01-build.sh`) handles both Rust compilation and ZK program building
   - Rust compilation happens in the I Ching program directory
   - ZK program building happens from the project root
   - You don't need to run any manual build steps

4. **AWS Configuration**: The S3 bucket must exist and be accessible with your credentials

Each script will guide you to the next step in the process. 