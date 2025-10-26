<script>
  // Example form with validation states
  let formData = $state({
    username: '',
    email: '',
    password: '',
    confirmPassword: '',
    agreeTerms: false,
    userRole: 'user',
    bio: '',
    notifications: true
  });

  let validation = $state({
    username: { valid: null, message: '' },
    email: { valid: null, message: '' },
    password: { valid: null, message: '' },
    confirmPassword: { valid: null, message: '' }
  });

  let passwordStrength = $state(0); // 0: weak, 1: medium, 2: strong

  function validateUsername() {
    if (formData.username.length === 0) {
      validation.username = { valid: null, message: '' };
    } else if (formData.username.length < 3) {
      validation.username = { valid: false, message: 'Username must be at least 3 characters' };
    } else if (!/^[a-zA-Z0-9_]+$/.test(formData.username)) {
      validation.username = { valid: false, message: 'Only letters, numbers, and underscores allowed' };
    } else {
      validation.username = { valid: true, message: 'Username is available' };
    }
  }

  function validateEmail() {
    if (formData.email.length === 0) {
      validation.email = { valid: null, message: '' };
    } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(formData.email)) {
      validation.email = { valid: false, message: 'Please enter a valid email address' };
    } else {
      validation.email = { valid: true, message: 'Email is valid' };
    }
  }

  function validatePassword() {
    if (formData.password.length === 0) {
      validation.password = { valid: null, message: '' };
      passwordStrength = 0;
    } else if (formData.password.length < 8) {
      validation.password = { valid: false, message: 'Password must be at least 8 characters' };
      passwordStrength = 0;
    } else {
      // Calculate password strength
      let strength = 0;
      if (formData.password.length >= 12) strength++;
      if (/[A-Z]/.test(formData.password)) strength++;
      if (/[0-9]/.test(formData.password)) strength++;
      if (/[^A-Za-z0-9]/.test(formData.password)) strength++;
      
      passwordStrength = strength <= 1 ? 0 : strength <= 2 ? 1 : 2;
      
      validation.password = { 
        valid: strength >= 2, 
        message: strength >= 2 ? 'Strong password' : 'Password should be stronger' 
      };
    }
    validateConfirmPassword();
  }

  function validateConfirmPassword() {
    if (formData.confirmPassword.length === 0) {
      validation.confirmPassword = { valid: null, message: '' };
    } else if (formData.password !== formData.confirmPassword) {
      validation.confirmPassword = { valid: false, message: 'Passwords do not match' };
    } else {
      validation.confirmPassword = { valid: true, message: 'Passwords match' };
    }
  }

  function handleSubmit(e) {
    e.preventDefault();
    console.log('Form submitted:', formData);
  }
</script>

<div class="form-example-container">
  <div class="card bg-base-100 shadow-xl max-w-2xl mx-auto">
    <div class="card-body">
      <h2 class="card-title text-2xl mb-4">
        <i class="bi bi-person-plus"></i>
        Create Account
      </h2>

      <form on:submit={handleSubmit}>
        <!-- Form Section: Account Details -->
        <div class="form-section">
          <div class="form-section-title">
            <i class="bi bi-person-badge"></i>
            Account Details
          </div>

          <div class="form-grid form-grid-2">
            <!-- Username with validation -->
            <div class="form-control">
              <label class="label">
                <span class="label-text">
                  Username
                  <span class="required">*</span>
                </span>
              </label>
              <div class="input-group">
                <i class="input-icon-left bi bi-person"></i>
                <input
                  type="text"
                  bind:value={formData.username}
                  on:input={validateUsername}
                  placeholder="johndoe"
                  class="input input-bordered input-primary input-with-icon-left w-full"
                  class:input-success={validation.username.valid === true}
                  class:input-error={validation.username.valid === false}
                />
                {#if validation.username.valid !== null}
                  <i class="input-icon-right bi bi-{validation.username.valid ? 'check-circle-fill' : 'x-circle-fill'}"
                     class:field-icon-success={validation.username.valid}
                     class:field-icon-error={!validation.username.valid}></i>
                {/if}
              </div>
              {#if validation.username.message}
                <div class="validation-message validation-message-{validation.username.valid ? 'success' : 'error'}">
                  <i class="bi bi-{validation.username.valid ? 'check-circle-fill' : 'exclamation-circle-fill'}"></i>
                  <span>{validation.username.message}</span>
                </div>
              {/if}
            </div>

            <!-- Email with validation -->
            <div class="form-control">
              <label class="label">
                <span class="label-text">
                  Email
                  <span class="required">*</span>
                </span>
              </label>
              <div class="input-group">
                <i class="input-icon-left bi bi-envelope"></i>
                <input
                  type="email"
                  bind:value={formData.email}
                  on:input={validateEmail}
                  placeholder="john@example.com"
                  class="input input-bordered input-primary input-with-icon-left w-full"
                  class:input-success={validation.email.valid === true}
                  class:input-error={validation.email.valid === false}
                />
                {#if validation.email.valid !== null}
                  <i class="input-icon-right bi bi-{validation.email.valid ? 'check-circle-fill' : 'x-circle-fill'}"
                     class:field-icon-success={validation.email.valid}
                     class:field-icon-error={!validation.email.valid}></i>
                {/if}
              </div>
              {#if validation.email.message}
                <div class="validation-message validation-message-{validation.email.valid ? 'success' : 'error'}">
                  <i class="bi bi-{validation.email.valid ? 'check-circle-fill' : 'exclamation-circle-fill'}"></i>
                  <span>{validation.email.message}</span>
                </div>
              {/if}
            </div>
          </div>

          <!-- Password with strength indicator -->
          <div class="form-control">
            <label class="label">
              <span class="label-text">
                Password
                <span class="required">*</span>
              </span>
            </label>
            <div class="input-group">
              <i class="input-icon-left bi bi-lock"></i>
              <input
                type="password"
                bind:value={formData.password}
                on:input={validatePassword}
                placeholder="••••••••"
                class="input input-bordered input-primary input-with-icon-left w-full"
                class:input-success={validation.password.valid === true}
                class:input-error={validation.password.valid === false}
              />
              {#if validation.password.valid !== null}
                <i class="input-icon-right bi bi-{validation.password.valid ? 'shield-check' : 'shield-exclamation'}"
                   class:field-icon-success={validation.password.valid}
                   class:field-icon-error={!validation.password.valid}></i>
              {/if}
            </div>
            {#if formData.password.length > 0}
              <div class="password-strength">
                <div class="password-strength-bar password-strength-{passwordStrength === 0 ? 'weak' : passwordStrength === 1 ? 'medium' : 'strong'}"></div>
              </div>
              <div class="input-help">
                <i class="bi bi-info-circle"></i>
                <span>{passwordStrength === 0 ? 'Weak' : passwordStrength === 1 ? 'Medium' : 'Strong'} password</span>
              </div>
            {/if}
            {#if validation.password.message}
              <div class="validation-message validation-message-{validation.password.valid ? 'success' : 'warning'}">
                <i class="bi bi-{validation.password.valid ? 'check-circle-fill' : 'exclamation-triangle-fill'}"></i>
                <span>{validation.password.message}</span>
              </div>
            {/if}
          </div>

          <!-- Confirm Password -->
          <div class="form-control">
            <label class="label">
              <span class="label-text">
                Confirm Password
                <span class="required">*</span>
              </span>
            </label>
            <div class="input-group">
              <i class="input-icon-left bi bi-lock-fill"></i>
              <input
                type="password"
                bind:value={formData.confirmPassword}
                on:input={validateConfirmPassword}
                placeholder="••••••••"
                class="input input-bordered input-primary input-with-icon-left w-full"
                class:input-success={validation.confirmPassword.valid === true}
                class:input-error={validation.confirmPassword.valid === false}
              />
              {#if validation.confirmPassword.valid !== null}
                <i class="input-icon-right bi bi-{validation.confirmPassword.valid ? 'check-circle-fill' : 'x-circle-fill'}"
                   class:field-icon-success={validation.confirmPassword.valid}
                   class:field-icon-error={!validation.confirmPassword.valid}></i>
              {/if}
            </div>
            {#if validation.confirmPassword.message}
              <div class="validation-message validation-message-{validation.confirmPassword.valid ? 'success' : 'error'}">
                <i class="bi bi-{validation.confirmPassword.valid ? 'check-circle-fill' : 'exclamation-circle-fill'}"></i>
                <span>{validation.confirmPassword.message}</span>
              </div>
            {/if}
          </div>
        </div>

        <!-- Form Section: Profile -->
        <div class="form-section">
          <div class="form-section-title">
            <i class="bi bi-person-circle"></i>
            Profile Information
          </div>

          <div class="form-control">
            <label class="label">
              <span class="label-text">User Role</span>
            </label>
            <select bind:value={formData.userRole} class="select select-bordered select-primary w-full">
              <option value="user">User</option>
              <option value="moderator">Moderator</option>
              <option value="admin">Administrator</option>
            </select>
          </div>

          <div class="form-control">
            <label class="label">
              <span class="label-text">Bio</span>
            </label>
            <textarea
              bind:value={formData.bio}
              placeholder="Tell us about yourself..."
              class="textarea textarea-bordered textarea-primary w-full"
              rows="4"
            ></textarea>
            <div class="input-help">
              <i class="bi bi-info-circle"></i>
              <span>{formData.bio.length}/500 characters</span>
            </div>
          </div>
        </div>

        <!-- Form Section: Preferences -->
        <div class="form-section">
          <div class="form-section-title">
            <i class="bi bi-gear"></i>
            Preferences
          </div>

          <div class="form-control">
            <label class="label cursor-pointer justify-start gap-3">
              <input type="checkbox" bind:checked={formData.notifications} class="toggle toggle-primary" />
              <div>
                <span class="label-text font-semibold">Email Notifications</span>
                <div class="label-text-alt">Receive email updates about your account activity</div>
              </div>
            </label>
          </div>

          <div class="form-control">
            <label class="label cursor-pointer justify-start gap-3">
              <input type="checkbox" bind:checked={formData.agreeTerms} class="checkbox checkbox-primary" />
              <div>
                <span class="label-text">
                  I agree to the Terms and Conditions
                  <span class="required">*</span>
                </span>
              </div>
            </label>
          </div>
        </div>

        <!-- Form Actions -->
        <div class="card-actions justify-end gap-3 mt-6">
          <button type="button" class="btn btn-outline">
            <i class="bi bi-x-lg"></i>
            Cancel
          </button>
          <button 
            type="submit" 
            class="btn btn-primary"
            disabled={!formData.agreeTerms || !validation.username.valid || !validation.email.valid || !validation.password.valid || !validation.confirmPassword.valid}
          >
            <i class="bi bi-check-lg"></i>
            Create Account
          </button>
        </div>
      </form>
    </div>
  </div>
</div>

<style>
  .form-example-container {
    padding: 2rem;
    min-height: 100vh;
    background: linear-gradient(135deg, hsl(var(--b2)) 0%, hsl(var(--b3)) 100%);
  }
</style>
