import React, { useRef, useMemo } from 'react';
import ReactQuill from 'react-quill-new';
import 'react-quill-new/dist/quill.snow.css';
import { uploadMedia } from '../api';

/**
 * A robust Rich Editor supporting inline image injection automatically pushed to backend.
 * Designed as a drop-in replacement for antd <Input /> inside <Form.Item />.
 */
const RichEditor = ({ value, onChange, placeholder = 'Draft content...' }) => {
    const quillRef = useRef(null);

    const imageHandler = () => {
        const input = document.createElement('input');
        input.setAttribute('type', 'file');
        input.setAttribute('accept', 'image/*,video/*');
        input.click();

        input.onchange = async () => {
            const file = input.files ? input.files[0] : null;
            if (!file) return;

            try {
                const res = await uploadMedia(file);
                const quill = quillRef.current.getEditor();
                const range = quill.getSelection();
                
                // res.url is standard relative resolved path e.g., "/uploads/uuid.png"
                quill.insertEmbed(range.index, 'image', res.url);
            } catch (error) {
                console.error('Failed to inject media payload:', error);
            }
        };
    };

    const modules = useMemo(() => ({
        toolbar: {
            container: [
                [{ header: [1, 2, false] }],
                ['bold', 'italic', 'underline', 'strike', 'blockquote'],
                [{ list: 'ordered' }, { list: 'bullet' }],
                ['link', 'image'],
                ['clean'],
            ],
            handlers: {
                image: imageHandler,
            },
        },
    }), []);

    return (
        <div className="rich-editor-container" style={{ minHeight: '200px' }}>
            <ReactQuill
                ref={quillRef}
                theme="snow"
                value={value || ''}
                onChange={onChange}
                modules={modules}
                placeholder={placeholder}
                style={{ height: 'auto' }}
            />
        </div>
    );
};

export default RichEditor;
