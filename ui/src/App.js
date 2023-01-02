import * as React from 'react';
import Box from '@mui/material/Box';

import Container from '@mui/material/Container';
import Button from '@mui/material/Button';
import {Grid, TextField, List, ListItem, ListItemText, Pagination} from '@mui/material';
import axios from "axios";
import {throttle} from 'lodash';

export default class App extends React.PureComponent {

    state = {
        pageSize: 10,
        pageIndex: 0,
        pageTotal: 0,
        searchValue: "",
        searchResult: []
    }

    onChange = e => {
        this.setState({searchValue: e.target.value});
        if (!e.target.value) {
            this.setState({
                pageIndex: 0,
                pageTotal: 0,
                searchResult: []
            })
        }
        let throttled = throttle(() => this.handleSearch(e.target.value, 1), 50);
        throttled();
    }

    handleSearch = (value, page_index) => {
        if (!value) {
            return
        }
        axios({
            method: "get",
            url: "http://localhost:8080/search",
            params: {
                q: value,
                p: page_index,
                ps: this.state.pageSize
            },
        }).then(response => {
            this.setState({
                pageIndex: response.data.page_info.page_index,
                pageTotal: response.data.page_info.page_total,
                searchResult: [...response.data.data]
            })
        });
    };

    handlePageChange = (e, value) => {
        this.setState({
            pageIndex: value
        });
        this.handleSearch(this.state.searchValue, value);
    };

    render() {
        const {searchResult, pageTotal, pageIndex} = this.state;
        return (
            <Box
                marginTop={2}
            >
                <Container>
                    <Grid item xl={8} xs={10} alignContent='center'>
                        <TextField
                            id='outlined-basic'
                            label='type something here'
                            fullWidth
                            size='small'
                            value={this.state.searchValue}
                            onChange={this.onChange}
                        />
                    </Grid>
                    {
                        searchResult.length !== 0 ? (<Grid> <List dense>
                            {searchResult.map((r) => {
                                return (
                                    <ListItem key={r._id.$oid} secondaryAction={<Button href={r.wiki_page}>VIEW</Button>}>
                                        <ListItemText>{r.title}</ListItemText>
                                    </ListItem>
                                );
                            })}
                        </List>
                            <Pagination page={pageIndex} count={pageTotal} onChange={this.handlePageChange}
                                        showFirstButton showLastButton/></Grid>) : <div></div>
                    }

                </Container>
            </Box>
        );
    }
}
