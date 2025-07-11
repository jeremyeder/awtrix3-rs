class Awtrix3 < Formula
  desc "Modern, async Rust CLI for controlling AWTRIX3 LED matrix displays"
  homepage "https://github.com/jeremyeder/awtrix3-rs"
  license "MIT OR Apache-2.0"
  head "https://github.com/jeremyeder/awtrix3-rs.git", branch: "main"

  on_macos do
    if Hardware::CPU.intel?
      url "https://github.com/jeremyeder/awtrix3-rs/releases/latest/download/awtrix3-latest-x86_64-apple-darwin.tar.gz"
      sha256 "" # Will be updated automatically by release process
    elsif Hardware::CPU.arm?
      url "https://github.com/jeremyeder/awtrix3-rs/releases/latest/download/awtrix3-latest-aarch64-apple-darwin.tar.gz"
      sha256 "" # Will be updated automatically by release process
    end
  end

  depends_on "rust" => :build if build.head?

  def install
    if build.head?
      system "cargo", "install", *std_cargo_args
      
      # Generate and install shell completions
      generate_completions_from_executable(bin/"awtrix3", "completions")
    else
      bin.install "awtrix3"
      
      # Install shell completions if available
      if (buildpath/"completions").exist?
        bash_completion.install "completions/awtrix3.bash" => "awtrix3"
        zsh_completion.install "completions/_awtrix3"
        fish_completion.install "completions/awtrix3.fish"
      end
      
      # Install examples and documentation
      if (buildpath/"examples").exist?
        doc.install "examples"
      end
    end
  end

  test do
    # Test that the binary runs and shows version
    assert_match version.to_s, shell_output("#{bin}/awtrix3 --version")
    
    # Test that help command works
    help_output = shell_output("#{bin}/awtrix3 --help")
    assert_match "Modern CLI for AWTRIX3", help_output
    
    # Test configuration directory creation
    system bin/"awtrix3", "device", "list"
    assert_predicate testpath/".config/awtrix3", :exist?
  end
end