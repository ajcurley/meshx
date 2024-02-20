package exchange

import (
	"bufio"
	"bytes"
	"compress/gzip"
	"errors"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"unicode"
	"unicode/utf8"

	"github.com/ajcurley/meshx"
)

const (
	PrefixVertex = "v"
	PrefixFace   = "f"
	PrefixGroup  = "g"
)

var (
	ErrInvalidVertex = errors.New("invalid vertex")
	ErrInvalidFace   = errors.New("invalid face")
)

// OBJReader manages parsing an OBJ (WaveFront) file. This supports both ASCII
// and GZIP ASCII files.
type OBJReader struct {
	reader      io.Reader
	vertices    []meshx.Vector
	faces       []int
	faceOffsets []int
	facePatches []int
	patches     []string
}

// Construct an OBJ reader from an io.Reader interface.
func NewOBJReader(reader io.Reader) *OBJReader {
	return &OBJReader{
		reader:      reader,
		vertices:    make([]meshx.Vector, 0),
		faces:       make([]int, 0),
		faceOffsets: make([]int, 0),
		facePatches: make([]int, 0),
		patches:     make([]string, 0),
	}
}

// Read an OBJ file from a file path.
func ReadOBJFromPath(path string) (*OBJReader, error) {
	file, err := os.Open(path)
	if err != nil {
		return nil, err
	}
	defer file.Close()

	var reader io.Reader

	if strings.ToLower(filepath.Ext(path)) == ".gz" {
		gzipFile, err := gzip.NewReader(file)
		if err != nil {
			return nil, err
		}
		reader = gzipFile
	} else {
		reader = file
	}

	objReader := NewOBJReader(reader)

	if err := objReader.Read(); err != nil {
		return nil, err
	}

	return objReader, nil
}

// Read the OBJ file.
func (r *OBJReader) Read() error {
	count := 1
	reader := bufio.NewReader(r.reader)

	for {
		data, err := reader.ReadBytes('\n')
		if errors.Is(err, io.EOF) {
			break
		}

		data = bytes.TrimSpace(data)
		prefix := r.parsePrefix(data)

		switch string(prefix) {
		case PrefixVertex:
			err = r.parseVertex(data)
		case PrefixFace:
			err = r.parseFace(data)
		case PrefixGroup:
			r.parseGroup(data)
		}

		if err != nil {
			return fmt.Errorf("line %d: %v", count, err)
		}

		count++
	}

	return nil
}

// Parse a prefix from a line.
func (r *OBJReader) parsePrefix(data []byte) []byte {
	for i := 0; i < len(data); i++ {
		value, _ := utf8.DecodeRune(data[i : i+1])

		if unicode.IsSpace(value) {
			return data[:i]
		}
	}
	return data
}

// Parse a vertex from a line.
func (r *OBJReader) parseVertex(data []byte) error {
	fields := bytes.Fields(data[len(PrefixVertex):])

	if len(fields) != 3 {
		return ErrInvalidVertex
	}

	var values [3]float64

	for i := 0; i < 3; i++ {
		value, err := strconv.ParseFloat(string(fields[i]), 64)
		if err != nil {
			return ErrInvalidVertex
		}

		values[i] = value
	}

	vertex := meshx.NewVectorFromArray(values)
	r.vertices = append(r.vertices, vertex)

	return nil
}

// Parse a face from a line.
func (r *OBJReader) parseFace(data []byte) error {
	fields := bytes.Fields(data[len(PrefixFace):])

	if len(fields) <= 2 {
		return ErrInvalidFace
	}

	faceOffset := len(r.faces)

	for i := 0; i < len(fields); i++ {
		if idx := bytes.IndexByte(fields[i], byte('/')); idx != -1 {
			fields[i] = fields[i][:idx]
		}

		value, err := strconv.Atoi(string(fields[i]))
		if err != nil || value <= 0 {
			return ErrInvalidFace
		}

		r.faces = append(r.faces, value-1)
	}

	r.faceOffsets = append(r.faceOffsets, faceOffset)
	r.facePatches = append(r.facePatches, len(r.patches)-1)

	return nil
}

// Parse a group from a line.
func (r *OBJReader) parseGroup(data []byte) {
	group := bytes.TrimSpace(data[len(PrefixGroup):])
	patch := string(group)
	r.patches = append(r.patches, patch)
}

// Get a vertex by index.
func (r *OBJReader) GetVertex(index int) meshx.Vector {
	return r.vertices[index]
}

// Get the number of vertices.
func (r *OBJReader) GetNumberOfVertices() int {
	return len(r.vertices)
}

// Get a face by index.
func (r *OBJReader) GetFace(index int) []int {
	if index == r.GetNumberOfFaces()-1 {
		faceStart := r.faceOffsets[index]
		return r.faces[faceStart:]
	}

	faceStart := r.faceOffsets[index]
	faceEnd := r.faceOffsets[index+1]
	return r.faces[faceStart:faceEnd]
}

// Get a face patch by index.
func (r *OBJReader) GetFacePatch(index int) int {
	return r.facePatches[index]
}

// Get the number of faces.
func (r *OBJReader) GetNumberOfFaces() int {
	return len(r.faceOffsets)
}

// Get a patch by index.
func (r *OBJReader) GetPatch(index int) string {
	return r.patches[index]
}

// Get the number of patches.
func (r *OBJReader) GetNumberOfPatches() int {
	return len(r.patches)
}